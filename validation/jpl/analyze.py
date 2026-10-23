#!/usr/bin/env python3
"""Interpret H's recorded observations; never replaces the canonical exporter.

Counterparts: jpleph.c interp, jpl_state cache transition, jpl_pleph Earth/Moon
combination. Exact rational Chebyshev polynomials are an independent diagnostic.
No production Rust budgets or astronomical truth claims are established here.
"""
from fractions import Fraction
from math import factorial
import json
import math
import struct
from pathlib import Path

HERE=Path(__file__).resolve().parent


def chebyshev(n,x,derivative):
    if n==0: return Fraction(1 if derivative==0 else 0)
    total=Fraction(0)
    for m in range(n//2+1):
        power=n-2*m
        if power>=derivative:
            coefficient=Fraction(n*(-1)**m*factorial(n-m-1)*2**power,
                                 2*factorial(m)*factorial(power))
            total+=coefficient*Fraction(factorial(power),factorial(power-derivative))*x**(power-derivative)
    return total


def main():
    data=json.loads((HERE/"outputs/reference.json").read_text())
    cases=json.loads((HERE/"cases.json").read_text())
    inputs=json.loads((HERE/"inputs.json").read_text())
    result={"scope":"host observations except isolated exact rational helper diagnostic; no Rust budgets"}
    errors=[0.0,0.0,0.0]; scales=[0.0,0.0,0.0]; count=0
    for command,event in zip(cases["helper_interpolation"]["commands"],data["helper_interpolation"][1:]):
        if not command.startswith("interp "): continue
        _,fraction,step,ncf,ncm,na,velocity=command.split()
        fraction,step=Fraction(fraction),Fraction(step)
        ncf,ncm,na,velocity=map(int,(ncf,ncm,na,velocity))
        if not step: continue
        temp=fraction*na; sub=int(temp); tc=2*(temp-sub)-1
        if sub==na: sub-=1; tc=Fraction(1)
        for d in range(velocity):
            for component in range(ncm):
                expected=Fraction(0)
                for k in range(ncf):
                    i=ncf*(sub*ncm+component)+k
                    coefficient=Fraction((-1 if i%7==0 else 1)*(i+1),16)
                    expected+=coefficient*chebyshev(k,tc,d)
                expected*=Fraction(na*2,step)**d
                observed=float(event["out"][d*ncm+component]["decimal"])
                errors[d]=max(errors[d],abs(observed-float(expected)))
                scales[d]=max(scales[d],abs(observed)); count+=1
        assert [x["decimal"] for x in event["out"][ncm*velocity:]]==[str(-9000-i) for i in range(ncm*velocity,9)]
    result["chebyshev_diagnostic"]={"method":"exact rational explicit polynomial sum differentiated analytically; rounded once for comparison",
        "components":count,"max_absolute_difference_position_velocity_acceleration":errors,
        "max_absolute_scale_position_velocity_acceleration":scales,
        "units":["synthetic coefficient units","coefficient units/day","coefficient units/day^2"]}
    result["cache_sequences"]={}
    for name in ("de405_truncated_state","de405_truncated_pleph","partial_cache_le","partial_cache_be","seek_failure_recovery"):
        records=[e for e in data[name] if e["op"] in ("state","pleph")]
        statuses=[e["status"] for e in records]
        assert statuses==([0,-2,0,0] if name.startswith("de405") else [0,-6,0,0] if name.startswith("seek") else [0,-2,0,0,0,0])
        if records[0]["op"]=="state":
            assert records[1]["pv"]==[{"bits":struct.pack(">d",-9000.0-i).hex(),"class":"finite","decimal":str(-9000-i)} for i in range(96)]
        result["cache_sequences"][name]={"statuses":statuses,"cache_locations":[e["cache_loc"] for e in records],
            "x_observations":[e.get("pv",e.get("out",[None]))[0] for e in records],
            "same_io_counts_failure_and_repeat":records[1]["io_calls"]==records[2]["io_calls"]}
        assert records[1]["io_calls"]==records[2]["io_calls"]
        if name.startswith("partial"):
            suffix=name[-2:]; recsize=inputs[f"extended_{suffix}.bin"]["specification"]["recsize"]
            fixture=(HERE/"build/inputs"/f"extended_{suffix}.bin").read_bytes()
            size,requested,returned=records[1]["last_fread_size_requested_returned"]
            assert size==8 and requested*8==recsize
            # This host's stdio read-ahead can survive truncation. Compare the
            # actual complete read prefix rather than guessing it from EOF.
            prefix=returned*size
            expected=fixture[3*recsize:3*recsize+prefix]+bytes.fromhex(records[0]["cache_bytes"])[prefix:]
            assert bytes.fromhex(records[1]["cache_bytes"])==expected
            result["cache_sequences"][name]["complete_bytes_read"]=prefix
            assert records[1]["cache_bytes"]==records[2]["cache_bytes"]==records[3]["cache_bytes"]
            assert records[4]["pv"]==records[0]["pv"]
        elif name.startswith("de405"):
            key="pv" if records[0]["op"]=="state" else "out"
            assert records[0][key]==records[-1][key]
    # Compare the full observed Earth vector against the state-level EMB/Moon
    # combination, including stale post-short-read coefficients; error is not
    # an assertion of portable behavior beyond the pointer UB in jpl_state.
    direct=[e for e in data["de405_truncated_state"] if e["op"]=="state"]
    pleph=[e for e in data["de405_truncated_pleph"] if e["op"]=="pleph"]
    differences=[]; nonfinite_classes=[]
    for s,p in zip(direct,pleph):
        if s["status"]: assert "out" not in p; continue
        for i in range(6):
            expected=float(s["pv"][12+i]["decimal"])-float(s["pv"][54+i]["decimal"])/(1+81.30056)
            observed=float(p["out"][i]["decimal"])
            if math.isfinite(expected):
                assert math.isfinite(observed)
                differences.append(abs(expected-observed))
            else:
                kind="NaN" if math.isnan(expected) else "+Infinity" if expected>0 else "-Infinity"
                assert p["out"][i]["class"]==kind
                nonfinite_classes.append(kind)
    result["de405_api_combination_max_abs_difference"]=max(differences)
    result["de405_api_combination_finite_components"]=len(differences)
    result["de405_api_combination_nonfinite_classes"]=nonfinite_classes
    result["file_lifetime"]={}
    for suffix in ("le","be"):
        for mutation in ("replace","overwrite"):
            name=f"lifetime_{mutation}_{suffix}"
            records=[e for e in data[name] if e["op"]=="state"]
            change=next(e for e in data[name] if e["op"]==mutation)
            assert records[0]["pv"]==records[1]["pv"]
            assert change["path_matches_open_inode"]==(mutation=="overwrite")
            assert records[0]["pv"]!=records[-1]["pv"]
            assert (records[0]["pv"]==records[3]["pv"])==(mutation=="replace")
            result["file_lifetime"][name]={"path_matches_open_inode":change["path_matches_open_inode"],
                "cached_value_unchanged":True,"reload_observes_replacement":mutation=="overwrite",
                "reopen_observes_replacement":True}
    result["init_statuses"]={name:[{"nonnull":e["nonnull"],"status":e["init_error"]} for e in events if e["op"]=="open"]
        for name,events in data.items() if name.startswith("init_")}
    for names in (("state_sequences_le","state_sequences_be"),("pleph_pairs_le","pleph_pairs_be")):
        # Exclude byte-level cache/storage fields and header swap flag.
        for a,b in zip(data[names[0]],data[names[1]]):
            for key in ("pv","nut","out","sun","status"):
                if key in a: assert a[key]==b[key],(names,a.get("event"),key)
    result["ordinary_endian_numerical_agreement"]="exact finite bits/nonfinite classes in state_sequences and pleph_pairs"
    (HERE/"outputs/analysis.json").write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
    print(json.dumps({"status":"PASS","chebyshev_diagnostic":result["chebyshev_diagnostic"],
                      "de405_api_combination_max_abs_difference":max(differences),
                      "de405_api_combination_finite_components":len(differences),
                      "de405_api_combination_nonfinite_classes":nonfinite_classes},indent=2))


if __name__=="__main__": main()
