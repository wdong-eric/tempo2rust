#!/usr/bin/env python3
"""Assignment H exporter/checker; counterparts: jpleph.c/h and jpl_int.h.

Synthetic encoding is diagnostic input, not an astronomical ephemeris.
Default checks the committed oracle; --record explicitly regenerates it.
Only validation/jpl is written. No third-party Python packages are needed.
"""
import argparse
import copy
import fnmatch
import hashlib
import json
import math
import platform
import re
import shutil
import struct
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SOURCE = "fda6ad4a9753ad7dc5d291406f2f310a9161423f"
BUILD = HERE / "build"
INPUTS = BUILD / "inputs"
RESULTS = HERE / "outputs"
REAL = ROOT / "tempo2/T2runtime/ephemeris/DE405.1950.2050"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def save(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def synthetic(order, version=430, count=402, optional=True, bad_extended=False,
              scanned=0, header_version=999, shift=0):
    """Encode all offsets explicitly; source infers version from title, not 2840."""
    start, step, records = 2450000.5, 32.0, 4
    ipt, cursor = [], 3
    for i in range(15):
        ncf, na = 4 + i % 5, (1, 2, 4, 8)[i % 4]
        if (i >= 13 and (version < 430 or count == 400)) or (i >= 11 and not optional):
            ipt.append([0, 0, 0])
        else:
            dim = 2 if i == 11 else 1 if i == 14 else 3
            ipt.append([cursor, ncf, na])
            cursor += ncf * na * dim
    ncoeff = cursor - 1
    recsize = ncoeff * 8
    assert recsize > max(2880 + max(0, count - 400) * 6, (count + scanned) * 8)
    data = bytearray(recsize * (records + 2))
    title = f"JPL Planetary Ephemeris DE{version}/LE{version}".encode()
    data[:84] = title.ljust(84, b" ")
    names = [f"C{i:05d}".encode() for i in range(count + scanned)]
    for i, name in enumerate(names):
        off = 252 + 6 * i if i < 400 else 2856 + 6 * (i - 400)
        data[off:off + 6] = name
    struct.pack_into(order + "3dI2d", data, 2652, start, start + records * step,
                     step, count, 149597870.691, 81.30056)
    struct.pack_into(order + "36I", data, 2696, *sum(ipt[:12], []))
    struct.pack_into(order + "I3I", data, 2840, header_version, *ipt[12])
    if version >= 430 and count != 400:
        extended = sum(ipt[13:], [])
        if bad_extended:
            extended[0] += 1
        struct.pack_into(order + "6I", data, 2856 + max(0, count - 400) * 6, *extended)
    for i in range(count + scanned):
        struct.pack_into(order + "d", data, recsize + 8 * i, i + 0.125)
    # All coefficients dyadic, distinguish record/body/subinterval/component/order.
    for record in range(records):
        off = (record + 2) * recsize
        struct.pack_into(order + "2d", data, off, start + record * step, start + (record + 1) * step)
        for i, (at, ncf, na) in enumerate(ipt):
            dim = 2 if i == 11 else 1 if i == 14 else 3
            for sub in range(na):
                for comp in range(dim):
                    base = (record + 1) * 1000 + (i + 1) * 10 + comp + sub / 16 + shift
                    for k in range(ncf):
                        c = base if k == 0 else (-1) ** k * base / 2 ** (k + 3)
                        index = at - 1 + ncf * (sub * dim + comp) + k
                        struct.pack_into(order + "d", data, off + 8 * index, c)
    spec = dict(byte_order=order, title_version=version, header_version_ignored=header_version,
                ncon=count, scanned_extra_names=scanned, optional=optional,
                bad_extended=bad_extended, shift=shift, start=start, step=step,
                records=records, ipt=ipt, ncoeff=ncoeff, recsize=recsize,
                coefficient_rule="base=(record+1)*1000+(ipt_index+1)*10+component+subinterval/16+shift; c0=base; ck=(-1)^k*base/2^(k+3)")
    return bytes(data), spec


def inputs():
    INPUTS.mkdir(parents=True, exist_ok=True)
    manifest = {}
    for order, suffix in (("<", "le"), (">", "be")):
        for name, kwargs in (("pre405", {"version": 405, "count": 6}),
                             ("extended", {}), ("small430", {"count": 6}),
                             ("absent", {"optional": False}),
                             ("bad_extended", {"bad_extended": True}),
                             ("scan400", {"count": 400, "scanned": 2}),
                             ("shifted", {"shift": 7000})):
            data, spec = synthetic(order, **kwargs)
            filename = f"{name}_{suffix}.bin"
            (INPUTS / filename).write_bytes(data)
            manifest[filename] = dict(sha256=sha(data), size=len(data), specification=spec)
    original = REAL.read_bytes()
    manifest["real_de405"] = dict(path=str(REAL.relative_to(ROOT)), size=len(original), sha256=sha(original))
    (INPUTS / "truncated_de405.bin").write_bytes(original[:-4000])
    manifest["truncated_de405.bin"] = dict(sha256=sha(original[:-4000]), size=len(original)-4000,
                                               construction="real_de405[:-4000]")
    (INPUTS / "empty.bin").write_bytes(b"")
    (INPUTS / "short_header.bin").write_bytes(b" " * 100)
    base = (INPUTS / "extended_le.bin").read_bytes()
    (INPUTS / "short_extended.bin").write_bytes(base[:2856 + 12 + 8])
    damaged = bytearray(base)
    struct.pack_into("<d", damaged, 2688, 90.0)
    (INPUTS / "bad_ratio.bin").write_bytes(damaged)
    recsize = manifest["extended_le.bin"]["specification"]["recsize"]
    (INPUTS / "short_constants.bin").write_bytes(base[:recsize + 15])
    (INPUTS / "zero_coefficients.bin").write_bytes(base[:recsize * 2] + bytes(len(base)-recsize*2))
    for name,offset,fmt,value in (("zero_au",2680,"d",0.0),
                                 ("nan_ratio",2688,"d",math.nan),
                                 ("threshold_count",2676,"I",65536),
                                 ("non_power_interval",2704,"I",3),
                                 ("large_coefficient_offset",2696,"I",10000000),
                                 ("tiny_step",2668,"d",1e-300)):
        damaged=bytearray(base); struct.pack_into("<"+fmt,damaged,offset,value)
        (INPUTS/f"{name}.bin").write_bytes(damaged)
    damaged=bytearray(base); damaged[26:84]=b"1"*58
    (INPUTS/"unterminated_title.bin").write_bytes(damaged)
    damaged=bytearray(base); damaged[26:84]=b" "*58
    (INPUTS/"title_zero.bin").write_bytes(damaged)
    for path in INPUTS.glob("*.bin"):
        if path.name not in manifest and not path.name.startswith(("working_", "replacement_")):
            manifest[path.name] = dict(sha256=sha(path.read_bytes()), size=path.stat().st_size)
    return manifest


def state(et, bary=1, entries=None):
    entries = entries if entries is not None else {0: 2, 2: 2, 9: 2}
    return "state " + format(et, ".17g") + f" {bary} " + " ".join(str(entries.get(i, 0)) for i in range(14))


def scenarios(manifest):
    cases = {}
    def add(name, commands, classification="controlled_state_host_observation", setup=None):
        cases[name] = dict(commands=commands, classification=classification, setup=setup or [])
    def op(name, flags=0):
        return f"open {INPUTS / name} {flags}"
    start, end = 2450000.5, 2450128.5
    for suffix in ("le", "be"):
        for name in ("pre405", "extended", "small430", "absent", "bad_extended", "scan400"):
            f = f"{name}_{suffix}.bin"
            count = manifest[f]["specification"]["ncon"] + manifest[f]["specification"]["scanned_extra_names"]
            commands = [op(f, 3)]
            commands += [f"getter d {v}" for v in (0, 8, 16)]
            commands += [f"getter l {v}" for v in (24, *range(44,224,4), 224,228,232,236,240)]
            commands += [f"constant {i}" for i in (-1, 0, count-1, count, 399,400,401)]
            commands += ["control", state(start + 3), "close"]
            add(f"layout_{name}_{suffix}", commands)
        f = f"extended_{suffix}.bin"
        commands = [op(f), "control"]
        # Endpoints, adjacent binary64 epochs, forward/backward and repeat.
        epochs = [start, math.nextafter(start, -math.inf), math.nextafter(start, math.inf),
                  start+4, math.nextafter(start+4, -math.inf), math.nextafter(start+4, math.inf),
                  start+32, math.nextafter(start+32, -math.inf), math.nextafter(start+32, math.inf),
                  end, math.nextafter(end, -math.inf), math.nextafter(end, math.inf), start+7, start+7]
        commands += [state(t) for t in epochs]
        # List/derivative/cache demand changes and barycentric conversion.
        for entries in ({}, {1:1}, {1:2}, {0:1,4:1,8:1}, {0:2,4:2,8:2}, {9:2},
                        {10:1}, {10:2}, {11:1}, {11:2}, {12:1}, {12:2}, {13:1}, {13:2},
                        {i:2 for i in range(14)}):
            commands += [state(start+7, b, entries) for b in (1,0)]
        commands += [state(start+7.25),state(start+7),"close"]
        add(f"state_sequences_{suffix}", commands)
        # Direct API statuses/outputs are retained as host observations: the
        # source pleph has private pv[13][6], see REPORT for pointer formation.
        commands = [op(f),"control"]
        for target in range(1,18):
            for velocity in (0,1):
                commands += [f"pleph {start+9} {target} {12 if target<14 else 0} {velocity}"]
        for target, centre in ((10,3),(3,10),(3,11),(11,3),(13,3),(3,13),(1,5),(5,1),
                               (0,1),(1,0),(18,1),(-1,1),(14,999),(18,18),(0,0)):
            commands += [f"pleph {start+9} {target} {centre} 1"]
        commands += [f"pleph {end+1} 1 1 1", "close"]
        commands.insert(-1,f"pleph {start+9} 1 12 -7")
        add(f"pleph_pairs_{suffix}", commands, "controlled_pleph_host_observation")
        add(f"absent_quantities_{suffix}",[op(f"absent_{suffix}.bin"),"control"]+
            [f"pleph {start+1} {i} 0 1" for i in range(14,18)]+[state(start+1,1,{i:2 for i in range(10,14)}),"close"])
        # A full old buffer precedes a short read; truncated bytes include a
        # partial binary64 word. Canonical fread skips byte swapping on error.
        recsize = manifest[f]["specification"]["recsize"]
        working = INPUTS / f"working_{suffix}.bin"
        setup = [[f, working.name]]
        add(f"partial_cache_{suffix}", [f"open {working} 0","control",state(start+3),
            f"truncate {working} {recsize*3+137}",state(start+40),state(start+40),
            f"overwrite {working} {INPUTS/f}",state(start+40),state(start+3),state(start+40),"close"], setup=setup)
        for size in (136,137):
            add(f"partial_far_{size}_{suffix}",[f"open {working} 0","control",state(start+3),
                f"truncate {working} {recsize*5+size}",state(start+104),state(start+104),"close"],setup=setup)
        add(f"constant_partial_{suffix}",[f"open {working} 0","control",f"truncate {working} {recsize+3}",
            "constant 0", "constant 1", "constant -1", "close"], setup=setup)
        shifted = f"shifted_{suffix}.bin"
        replace = INPUTS / f"replacement_{suffix}.bin"
        for mutation in ("replace", "overwrite"):
            target = replace if mutation == "replace" else INPUTS/shifted
            add(f"lifetime_{mutation}_{suffix}",[f"open {working} 0","control",state(start+3),
                f"{mutation} {working} {target}",state(start+3),state(start+70),state(start+3),"close",
                f"open {working} 0","control",state(start+3),"close"],
                setup=setup+([[shifted,replace.name]] if mutation=="replace" else []))
        # Validated bytes A, path replaced by valid B before native open: no race
        # needed to demonstrate that native consumes a separate file identity.
        add(f"u1_preflight_reopen_{suffix}",[f"replace {working} {replace}",f"open {working} 0","control",
                state(start+3),"close"],"preflight_identity_demonstration",setup+[[shifted,replace.name]])
    real_start, real_end = struct.unpack_from(">2d", REAL.read_bytes(), 2652)
    for api in ("state","pleph"):
        queries = [2450000.5,real_end-8,real_end-8,2450000.5]
        commands = [op("truncated_de405.bin"),"control"]
        commands += [state(t) if api=="state" else f"pleph {t} 3 12 1" for t in queries]
        commands += ["close"]
        add(f"de405_truncated_{api}",commands,
            "controlled_state_host_observation" if api=="state" else "host_observation_undefined_failure_outputs_suppressed")
    add("de405_full",[f"open {REAL} 3","control"]+
        [state(t,b,{i:2 for i in range(12)}) for t in (real_start,2450000.5,real_end) for b in (0,1)]+["close"])
    add("canonical_natural_status",[f"open {REAL} 0",f"pleph_status 2450000.5 3 12 1","close"],
        "unwritten_initializer_host_status_only")
    commands = []
    for f in ("missing.bin","empty.bin","short_header.bin","short_extended.bin","bad_ratio.bin"):
        commands += [op(f)]
    commands += [op("short_constants.bin",3),"close",op("extended_le.bin",3),"close"]
    add("init_failures",commands,"initializer_status_and_initialized_caller_buffers")
    add("init_optional_outputs",[op("small430_le.bin",1),"close",op("small430_le.bin",2),"close"],"initializer_caller_buffers")
    add("constant_failures",[op("small430_le.bin"),"fault read 1","constant 0",
        "fault read 2","constant 0","constant 0","fault seek 1","constant 1","constant 1","close"],"injected_libc_failure_initialized_caller_buffers")
    for kind,n in (("seek",1),("read",1),("read",2),("read",3),("read",4),("read",5),("alloc",1)):
        add(f"init_injected_{kind}_{n}",[f"fault {kind} {n}",op("extended_le.bin",3),
            *( ["close"] if (kind=="read" and n>=4) else []),op("extended_le.bin",3),"close"],"injected_libc_failure")
    add("seek_failure_recovery",[op("extended_le.bin"),"control",state(start+3),"fault seek 1",
        state(start+40),state(start+40),state(start+3),"close"],"injected_libc_failure_controlled_kernel")
    add("read_failure_before_first_success",[op("extended_le.bin"),"control","fault read 1",state(start+3),
        state(start+3),state(start+40),"close"],"injected_libc_failure_controlled_kernel")
    add("helper_interpolation",["helpers"]+
        [f"interp {t} 32 {ncf} {dim} {na} {vel}" for t in (0,0.125,0.5,1) for ncf in (1,2,5,17)
         for dim,na in ((1,1),(2,2),(3,8)) for vel in (1,2,3)]+
        ["interp 0.25 0 5 3 1 3","interp 0.25 0 1 3 1 3"],"initialized_helper_state")
    add("zero_coefficients",[op("zero_coefficients.bin"),"control",state(start+3),"close"])
    for name in ("zero_au","nan_ratio","non_power_interval","title_zero"):
        add(name,[op(name+".bin"),"control",state(start+3),"close"])
    add("threshold_count",[op("threshold_count.bin"),op("extended_le.bin"),"close"],"initializer_status")
    return cases


def execute(exe, case):
    for src,dest in case["setup"]:
        shutil.copyfile(INPUTS/src,INPUTS/dest)
    result=subprocess.run([str(exe)], input="\n".join(case["commands"])+"\n",text=True,capture_output=True,timeout=60)
    if result.returncode:
        raise RuntimeError((result.returncode,result.stderr,result.stdout[-1000:]))
    assert not result.stderr, result.stderr
    return [json.loads(line) for line in result.stdout.splitlines()]


def compare(actual,expected):
    # Full ordered structure comparison includes every value/class/status,
    # output slot and cache byte. NaN payload bits are deliberately absent.
    assert actual==expected, "reference mismatch (values/classes/statuses/order/cache bytes/coverage)"


def validate_roundtrip(value):
    if isinstance(value,dict):
        if value.get("class")=="finite":
            assert struct.pack(">d",float(value["decimal"])).hex()==value["bits"]
        for x in value.values(): validate_roundtrip(x)
    elif isinstance(value,list):
        for x in value: validate_roundtrip(x)


def mutation_checks(actual):
    checks={}
    state_key="state_sequences_le"
    first=next(i for i,x in enumerate(actual[state_key]) if x["op"]=="state")
    def check(name,mutate):
        changed=copy.deepcopy(actual); mutate(changed)
        try: compare(changed,actual)
        except AssertionError: checks[name]="detected"
        else: raise AssertionError(f"mutation escaped: {name}")
    check("wrong_target_index",lambda x:x[state_key][first]["pv"].__setitem__(0,x[state_key][first]["pv"][6]))
    check("wrong_units",lambda x:x[state_key][first]["pv"][0].__setitem__("decimal","1"))
    check("wrong_result_class",lambda x:x[state_key][first]["pv"][0].__setitem__("class","NaN"))
    check("dropped_sequence_step",lambda x:x[state_key].pop(first))
    check("reordered_sequence",lambda x:x[state_key].__setitem__(slice(first,first+2),list(reversed(x[state_key][first:first+2]))))
    check("wrong_component_count",lambda x:x[state_key][first]["nut"].pop())
    check("changed_cache_transition",lambda x:x["partial_cache_be"][5].__setitem__("cache_loc",0))
    check("changed_cache_bytes",lambda x:x[state_key][first].__setitem__("cache_bytes","00"))
    check("wrong_error_status",lambda x:x["de405_truncated_state"][5].__setitem__("status",-2))
    return checks


def isolated_checks(source):
    """Terminating/invalid cases are subprocess diagnostics, never value oracles."""
    clang=shutil.which("clang"); assert clang
    commands={
        "ubsan":[clang,"-O1","-g","-std=c11","-fsanitize=undefined,address,float-cast-overflow","-fno-sanitize-recover=all"],
        "asan":[clang,"-O1","-g","-std=c11","-fsanitize=address","-fno-sanitize-recover=all"],
    }
    for name,args in commands.items():
        args += ["-I",str(source),str(HERE/"probe.c"),"-lm","-o",str(BUILD/name)]
        subprocess.run(args,capture_output=True,text=True,check=True)
    opening=f"open {INPUTS/'extended_le.bin'} 0"
    specs={
        "u2_packed_au":("ubsan",[opening,"getter d 28"],"misaligned address"),
        "u2_packed_emrat":("ubsan",[opening,"getter d 36"],"misaligned address"),
        "state_ipt_one_past":("ubsan",[opening,"control",state(2450003.5)],"index 15 out of bounds"),
        "pleph_ipt_one_past":("ubsan",[opening,"control","pleph 2450003.5 3 12 1"],"index 15 out of bounds"),
        "nan_epoch":("ubsan",[opening,"control",state(math.nan)],"outside the range"),
        "tiny_step_record_cast":("ubsan",[f"open {INPUTS/'tiny_step.bin'} 0","control",state(2450003.5)],"outside the range"),
        "max_cheby_assertion":("asan",["interp 0.5 32 18 3 1 2"],"Assertion"),
        "coefficient_offset":("asan",[f"open {INPUTS/'large_coefficient_offset.bin'} 0","control",state(2450003.5)],"AddressSanitizer"),
        "nut_four_slot_overwrite":("asan",[opening,"control","state_nut4"],"stack-buffer-overflow"),
        "unterminated_title":("asan",[f"open {INPUTS/'unterminated_title.bin'} 0"],"stack-buffer-overflow"),
    }
    results={}
    for name,(exe,lines,diagnostic) in specs.items():
        run=subprocess.run([str(BUILD/exe)],input="\n".join(lines)+"\n",text=True,capture_output=True,timeout=30)
        # Assertion text on macOS can be lower-case, unlike GNU libc.
        assert run.returncode and diagnostic.lower() in run.stderr.lower(),(name,run.returncode,run.stderr)
        completed=[json.loads(line) for line in run.stdout.splitlines() if line.endswith("}")]
        # Retain trace presentation as evidence; isolated_contract supplies the
        # stable classification and validates source locations when reported.
        summary=[line for line in run.stderr.splitlines() if "runtime error:" in line or "SUMMARY:" in line or "Assertion" in line or "Assertion failed" in line]
        summary=[re.sub(r"0x[0-9a-fA-F]+","<address>",line.replace(str(ROOT)+"/","")) for line in summary]
        results[name]=dict(returncode=run.returncode,diagnostic=diagnostic,summary=summary,
                           last_completed_op=completed[-1]["op"] if completed else None,
                           commands=lines,classification="undefined_or_terminating_no_numerical_oracle")
    return dict(compiler=subprocess.check_output([clang,"--version"],text=True).splitlines()[0],
                build_commands=commands,cases=results)


def isolated_contract(observed):
    """Gate failure kind/stage independently of ASan symbolizer availability."""
    source_lines={
        "u2_packed_au":77, "u2_packed_emrat":77,
        "state_ipt_one_past":625, "pleph_ipt_one_past":625,
        "nan_epoch":582, "tiny_step_record_cast":582,
        "max_cheby_assertion":326, "coefficient_offset":373,
        "nut_four_slot_overwrite":403, "unterminated_title":726,
    }
    contract=copy.deepcopy(observed)
    for name,case in contract["cases"].items():
        summary="\n".join(case.pop("summary"))
        asan=re.findall(r"SUMMARY: AddressSanitizer: ([\w-]+)",summary)
        assertion=re.findall(r"Assertion failed: \(([^\n]+)\), function",summary)
        if asan:
            assert len(asan)==1,(name,summary)
            kind="AddressSanitizer:"+asan[0]
        elif assertion:
            assert len(assertion)==1,(name,summary)
            kind="assertion:"+assertion[0]
        else:
            reasons={
                "misaligned address":"misaligned-access",
                "index 15 out of bounds":"array-index-out-of-bounds",
                "outside the range of representable values":"float-to-integer-overflow",
            }
            matched=[label for text,label in reasons.items()
                     if any(text in line for line in summary.splitlines() if "runtime error:" in line)]
            assert len(matched)==1,(name,"unclassified sanitizer failure",summary)
            kind="UndefinedBehaviorSanitizer:"+matched[0]
        # Inspect diagnostic summaries, not arbitrary caller stack frames.
        filename=r"([A-Za-z_][\w.-]*\.(?:c|cc|cpp|h))"
        locations=set(re.findall(r"\b"+filename+r":(\d+)",summary))
        locations.update(re.findall(r"file "+filename+r", line (\d+)",summary))
        expected={("jpleph.c",str(source_lines[name]))}
        if locations or not asan:
            assert locations==expected,(name,"unexpected/missing source location",locations,expected)
        assert case["returncode"]!=0,(name,"isolated failure returned success")
        case["failure_kind"]=kind
        case["source_location_if_reported"]=f"jpleph.c:{source_lines[name]}"
    return contract


def source_mutations(source,cc,cases,actual):
    """Build disposable source mutations; never alter the pinned copy or oracle."""
    text=(source/"jpleph.c").read_text()
    changes={
        "cache_after_success":("        eph->curr_cache_loc = nr;", "        /* experimental mutant: no early cache commit */", "partial_cache_be"),
        "wrong_au_units":("const double aufac = 1.0 / eph->au;","const double aufac = 1.0;","state_sequences_le"),
        "wrong_optional_dimension":("if( idx == 11)","if( idx == 10)","state_sequences_le"),
        "wrong_target_mapping":("pv[ntarg-1][i] - pv[ncent-1][i]","pv[ncent-1][i] - pv[ntarg-1][i]","pleph_pairs_le"),
        "wrong_finite_class":("*posvel = 0.0;","*posvel = NAN;","helper_interpolation"),
    }
    results={}
    for name,(before,after,case) in changes.items():
        assert text.count(before)==1,(name,text.count(before))
        directory=BUILD/"mutants"/name; directory.mkdir(parents=True,exist_ok=True)
        for h in ("jpleph.h","jpl_int.h"): shutil.copyfile(source/h,directory/h)
        altered=text.replace(before,after)
        if name=="cache_after_success":
            altered=altered.replace("        if( eph->swap_bytes)\n            swap_64_bit_val( buf, eph->ncoeff);",
                                    "        eph->curr_cache_loc = nr;\n        if( eph->swap_bytes)\n            swap_64_bit_val( buf, eph->ncoeff);")
        (directory/"jpleph.c").write_text(altered)
        exe=directory/"probe"
        command=[cc,"-O2","-std=c11","-I",str(directory),str(HERE/"probe.c"),"-lm","-o",str(exe)]
        subprocess.run(command,text=True,capture_output=True,check=True)
        observed=execute(exe,cases[case])
        assert observed!=actual[case],name
        first=next(i for i,(a,b) in enumerate(zip(observed,actual[case])) if a!=b)
        results[name]=dict(case=case,first_changed_event_index=first,status="detected",
                           mutant_sha256=sha((directory/"jpleph.c").read_bytes()))
    return results


def replay_prior(source,cc):
    prior=json.loads((ROOT/"validation/coverage/native_replay.json").read_text())
    saved=json.loads(subprocess.check_output(["git","-C",str(ROOT),"show",
        prior["rust_revision"]+":rust/verification/m1b/native_source_probe.json"]))
    probe=saved["probe_source"].encode()
    assert sha(probe)==prior["probe_source_sha256"]
    path=BUILD/"prior_probe.c"; path.write_bytes(probe)
    exe=BUILD/"prior_probe"
    command=[cc,"-O2","-I",str(source),str(path),str(source/"jpleph.c"),"-o",str(exe)]
    subprocess.run(command,capture_output=True,text=True,check=True)
    run=subprocess.run([str(exe),str(INPUTS/"truncated_de405.bin")],capture_output=True,text=True,check=True,timeout=30)
    assert run.stdout.strip()==prior["results"]["original"]["stdout"]
    return dict(probe_source=saved["probe_source"],probe_sha256=sha(probe),command=command,
        stdout=run.stdout,stderr=run.stderr,classification="unaltered_prior_probe_host_observation_not_portable_values")


def check_coverage(source,cases,isolated):
    coverage=json.loads((HERE/"coverage.json").read_text())
    functions=set(re.findall(r"^(?:static )?(?:void|int|long|double)\s*(?:\*\s*)?(?:DLL_FUNC\s+)?(\w+)\s*\(",
                             (source/"jpleph.c").read_text(),re.MULTILINE))
    def validate(document):
        assert set(document["functions"])==functions
        for entry in document["functions"].values():
            for pattern in entry["cases"]: assert fnmatch.filter(cases,pattern),pattern
            for name in entry["isolated"]: assert name in isolated["cases"],name
    validate(coverage)
    changed=copy.deepcopy(coverage); del changed["functions"]["interp"]
    try: validate(changed)
    except AssertionError: pass
    else: raise AssertionError("deleted function coverage escaped")
    return dict(implemented_functions=sorted(functions),missing_function_mutation="detected")


def main():
    parser=argparse.ArgumentParser(); parser.add_argument("--record",action="store_true"); args=parser.parse_args()
    BUILD.mkdir(exist_ok=True); source=BUILD/"source"; source.mkdir(exist_ok=True)
    source_hashes={}
    for filename in ("jpleph.c","jpleph.h","jpl_int.h"):
        blob=subprocess.check_output(["git","-C",str(ROOT/"tempo2"),"show",f"{SOURCE}:{filename}"])
        assert blob==(ROOT/"tempo2"/filename).read_bytes()
        (source/filename).write_bytes(blob); source_hashes[filename]=sha(blob)
    cc=shutil.which("gcc-16"); assert cc,"gcc-16 is required for the declared host profile"
    exe=BUILD/"probe"
    command=[cc,"-O2","-std=c11","-I",str(source),str(HERE/"probe.c"),"-lm","-o",str(exe)]
    compiled=subprocess.run(command,text=True,capture_output=True,check=True)
    manifest=inputs(); cases=scenarios(manifest)
    actual={name:execute(exe,case) for name,case in cases.items()}
    repeated={name:execute(exe,case) for name,case in cases.items()}
    compare(repeated,actual); validate_roundtrip(actual)
    mutation=mutation_checks(actual)
    live_mutations=source_mutations(source,cc,cases,actual)
    isolated_diagnostics=isolated_checks(source)
    isolated=isolated_contract(isolated_diagnostics)
    prior_replay=replay_prior(source,cc)
    coverage=check_coverage(source,cases,isolated)
    RESULTS.mkdir(exist_ok=True)
    if args.record:
        save(RESULTS/"reference.json",actual)
    compare(actual,json.loads((RESULTS/"reference.json").read_text()))
    # Portable commands use $INPUTS/$REAL markers in the declared manifest.
    portable=json.loads(json.dumps(cases).replace(str(INPUTS),"$INPUTS").replace(str(REAL),"$REAL"))
    for path,data in ((HERE/"inputs.json",manifest),(HERE/"cases.json",portable),(RESULTS/"isolated.json",isolated),
                      (RESULTS/"prior_replay.json",prior_replay)):
        if args.record: save(path,data)
        else: compare(data,json.loads(path.read_text()))
    # Trace formatting is evidence, never the exact reproducibility oracle.
    save(RESULTS/"isolated_diagnostics.json",isolated_diagnostics)
    prior=json.loads((ROOT/"validation/coverage/native_replay.json").read_text())
    assert prior["results"]["original"]["jpleph_sha256"]==source_hashes["jpleph.c"]
    assert prior["original_resource_sha256"]==manifest["real_de405"]["sha256"]
    assert prior["truncated_resource_sha256"]==manifest["truncated_de405.bin"]["sha256"]
    checks=dict(status="PASS",platform=platform.platform(),machine=platform.machine(),
                compiler=subprocess.check_output([cc,"--version"],text=True).splitlines()[0],
                compile_command=command,compile_stderr=compiled.stderr,
                source_commit=SOURCE,source_sha256=source_hashes,
                harness_sha256={name:sha((HERE/name).read_bytes()) for name in ("probe.c","run.py","analyze.py","test_isolated.py","diagnostic_fixtures.json","coverage.json","SOURCE_MAP.md")},
                prior_evidence={"path":"validation/coverage/native_replay.json","sha256":sha((ROOT/"validation/coverage/native_replay.json").read_bytes()),
                                "canonical_stdout":prior["results"]["original"]["stdout"],
                                "retained_stdout":prior["results"]["retained"]["stdout"]},
                cases=len(cases),events=sum(map(len,actual.values())),
                repeatability="two fresh processes per case; exact structured equality",mutations=mutation,
                source_mutations=live_mutations,
                coverage=coverage,
                reference_sha256=sha((RESULTS/"reference.json").read_bytes()))
    save(RESULTS/"checks.json",checks)
    print(json.dumps({k:checks[k] for k in ("status","cases","events","repeatability","mutations")},indent=2))


if __name__=="__main__": main()
