"""0007/0012/0071 public fitting gates; all arithmetic is decimal precision 70.

This compares exported quantities, not truth recovery or complete AE coverage.
Native TSV may repeat SVD records during robust iteration: the final solve owns
the reported spectrum, while native FitOutput owns covariance/errors.
"""
from decimal import Decimal, getcontext
from pathlib import Path
import json
import math
import re

getcontext().prec = 70
D = Decimal
ZERO = D(0)
# Source enum names are independent of candidate metadata. In particular, changing
# a column name must never bypass its physical-update checks.
_header = (Path(__file__).resolve().parents[2] / "tempo2/tempo2.h").read_text()
_enum = _header.split("enum label {", 1)[1].split("};", 1)[0]
PARAMETERS = re.findall(r"\bparam_\w+\b", re.sub(r"/\*.*?\*/|//[^\n]*", "", _enum, flags=re.S))


def decimal(value):
    if isinstance(value, dict):
        return D({"nan": "NaN", "-nan": "-NaN", "+infinity": "Infinity", "-infinity": "-Infinity"}[value["class"]])
    return D(str(value))


def checked_shape(ref, result):
    """Expected membership comes exclusively from the independent native export."""
    failures = []

    def validate(path, value, spec):
        def fail(reason):
            failures.append({"identity": path, "kind": "shape_or_identity", "reason": reason})
        if isinstance(spec, dict):
            if not isinstance(value, dict): return fail("expected object")
            for key, sub in spec.items():
                if key not in value: fail("missing " + key)
                else: validate(path + [key], value[key], sub)
        elif isinstance(spec, tuple):
            if spec[0] == "literal":
                if type(value) is not type(spec[1]) or value != spec[1]: fail("expected " + repr(spec[1]))
            else:
                n, sub = spec
                if not isinstance(value, list): return fail("expected array")
                if len(value) != n: fail("expected length " + str(n))
                for i, item in enumerate(value[:n]): validate(path + [i], item, sub[i] if isinstance(sub, list) else sub)
        elif spec == "number" or spec == "nullable_number":
            if value is None and spec == "nullable_number": return
            if not isinstance(value, (str, dict)): return fail("expected decimal string or numeric class")
            if isinstance(value, dict) and set(value) != {"class"}: return fail("invalid numeric class object")
            try:
                if decimal(value).is_snan(): fail("signaling NaN is not an exported class")
            except (ArithmeticError, ValueError, KeyError): fail("invalid numeric value")
        elif type(value) is not spec:
            fail("expected " + spec.__name__)

    literal = lambda v: ("literal", v)
    cycles = sorted({k[0] for k in ref})
    cycle_specs = []
    for it in cycles:
        at = lambda stage, p, i, key: ref[it, stage, p, i, key]
        psrs = sorted({k[2] for k in ref if k[0] == it and k[1] == "prepared"})
        phase_specs = {}
        for stage in ("prefit", "postfit"):
            phase_psrs = sorted({k[2] for k in ref if k[0] == it and k[1] == stage and k[4] == "residual"})
            assert phase_psrs == psrs, "native timing/fit pulsar membership disagrees"
            pulsar_specs=[]
            for p in psrs:
                observation={"canonical_index":int,"residual_s":"number","phase_turn":"number","bbat_mjd":"number"}
                for key,field,spec in [("pulse_number","pulse_number",str),("nphase","nphase_turn","number"),("sat","sat_mjd","number"),("deleted","deleted",int)]:
                    if (it,stage,p,0,key) in ref:observation[field]=spec
                for key,unit in [("frequency","MHz"),("uncertainty","us")]:
                    if (it,stage,p,0,key) in ref:observation[key]={"value":"number","unit":literal(unit)}
                pulsar_specs.append({"observations":(sum(k[0]==it and k[1]==stage and k[2]==p and k[4]=="residual"for k in ref),observation)})
            phase_specs[stage]={"pulsars":(len(psrs),pulsar_specs)}
        global_fit = any(k[0] == it and k[1] == "fit" and k[4] == "owner" and v == -1 for k, v in ref.items())
        groups = [psrs] if global_fit else [[p] for p in psrs]
        system_specs = []
        for group in groups:
            p0 = group[0]
            nf = int(at("fit", p0, -1, "total"))
            nc = sum(int(at("prepared", p, -1, "nconstraints")) for p in group)
            n = sum(int(at("prepared", p, -1, "ndata")) for p in group)
            col_specs = [{"pulsar": literal(int(at("fit", p0, j, "owner"))), "parameter": literal(PARAMETERS[int(at("fit", p0, j, "label"))]), "parameter_index": literal(int(at("fit", p0, j, "label"))), "subindex": literal(int(at("fit", p0, j, "counter")))} for j in range(nf)]
            row_specs = [{"pulsar": literal(p), "observation": literal(int(at("prepared", p, j, "index"))), "x_days": "number", "residual_s": "number", "sigma_s": "number"} for p in group for j in range(int(at("prepared", p, -1, "ndata")))]
            constraint_specs = [{"pulsar": literal(p), "label": literal(int(at("constraint", p, j, "label"))), "subindex": literal(int(at("constraint", p, j, "counter")))} for p in group for j in range(int(at("prepared", p, -1, "nconstraints")))]
            noise_members = sorted({(k[2], k[3]) for k in ref if k[0] == it and k[1] == "noise" and k[2] in group})
            noise_specs = [{"pulsar": literal(p), "observation": literal(i), **{key: "number" for key in ("red", "red_error", "dm", "dm_error", "chrom", "chrom_error", "solar_wind", "solar_wind_error")}} for p, i in noise_members]
            update_specs = []
            for p in group:
                for j in range(int(at("prepared", p, -1, "nparams"))):
                    label, counter = int(at("column", p, j, "label")), int(at("column", p, j, "counter"))
                    owner = -1 if any(at("fit", p0, c, "owner") == -1 and at("fit", p0, c, "label") == label and at("fit", p0, c, "counter") == counter for c in range(nf)) else p
                    physical_specs = {}
                    for when in ("before", "after", "postfit"):
                        for field in ("value", "error"):
                            flag = ref.get((it, when + "_update", p, label, "defined_" + field + "." + str(counter)))
                            if when == "postfit" and flag is None: continue
                            physical_specs[when + "_" + field] = (literal(None) if flag == 0 else "number") if flag is not None else ("number" if PARAMETERS[label] == "param_f" else "nullable_number")
                    update_specs.append({"pulsar": literal(p), "column": {"pulsar": literal(owner), "parameter": literal(PARAMETERS[label]), "parameter_index": literal(label), "subindex": literal(counter)}, **physical_specs, "increment": "number", "fit_error": "number"})
            control_specs = []
            for p in group:
                fields = {"pulsar": literal(p), "fit_mode": int, "sigma": (int(at("prepared", p, -1, "ndata")), "number"), "log_determinant": "number"}
                for key in ("fit_mode", "robust", "nits", "rescale_err_chisq"):
                    if (it, "prepared", p, -1, key) in ref:
                        fields[key] = literal(int(at("prepared", p, -1, key)))
                full = ref.get((it, "prepared", p, -1, "full_covariance"))
                if full is not None:
                    size = int(at("prepared", p, -1, "ndata"))
                    for key in ("covariance", "lower"):
                        fields[key] = (size, (size, "number")) if full else literal(None)
                control_specs.append(fields)
            solver_fields = {"singular_values": (nf, "number"), "retained": (nf, bool), "rank": int, "dof": int, "increments": (nf, "number"), "errors": (nf, "number"), "errors_unscaled": (nf, "number"), "error_multiplier": "number", "linearized_s": (n, "number"), "covariance": (nf, (nf, "number")), "chi_square": "number"}
            owner = -1 if global_fit else p0
            if (it, "solver_details", owner, -1, "solves") in ref:
                history_specs = []
                for h in sorted({int(k[1].split('.')[1]) for k in ref if k[0] == it and k[1].startswith('robust.') and k[2] == owner}):
                    final = bool(at("robust." + str(h), owner, -1, "final"))
                    history_specs.append({"iteration": literal(h), "phase": literal("final" if final else "scale"),
                        "scale": "number", "next_scale": literal(None) if final else "number",
                        "phi": literal(None) if h == 0 and not final else "number", "dphi": literal(None) if h == 0 and not final else "number",
                        "white_data": (n, "number"), "white_matrix": (n, (nf, "number")), "residual": (n, "number"),
                        "solver": {**solver_fields, "robust_history": literal([])}})
                solver_fields["robust_history"] = (len(history_specs), history_specs)
            system_specs.append({
                "columns": (nf, col_specs), "rows": (n, row_specs),
                "data": (n, "number"), "white_data": (n, "number"),
                "matrix": (n, (nf, "number")), "white_matrix": (n, (nf, "number")),
                "constraints": (nc, (nf, "number")), "constraint_targets": (nc, "number"), "constraint_rows": (nc, constraint_specs),
                "solver": solver_fields,
                "whitening": (len(group), control_specs),
                "noise": (len(noise_members), noise_specs), "updates": (len(update_specs), update_specs),
            })
        cycle_specs.append({"iteration": literal(it), **phase_specs, "systems": (len(groups), system_specs)})
    native_means=[v for k,v in ref.items() if k[1:]==("workflow",-1,-1,"remove_mean")]
    assert all(v in [0,1]for v in native_means), "invalid native mean convention"
    mean_controls=set(map(bool,native_means))
    assert len(mean_controls)<=1, "native workflow mean conventions disagree"
    remove_mean=next(iter(mean_controls),True)
    validate([], result, {"schema_version": literal("tempo2-general-fit-v1"), "fit_performed": literal(True), "remove_mean": literal(remove_mean), "cycles": (len(cycles), cycle_specs)})
    return failures


def compare(native, candidate, destination):
    ref = {}
    for line in Path(native).read_text().splitlines():
        cycle, stage, p, row, key, value = line.split("\t")
        ref[int(cycle), stage, int(p), int(row), key] = D(value)
    result = json.loads(Path(candidate).read_text())
    failures, stats, unchecked_physical = [], {}, set()
    shape_failures = checked_shape(ref, result)
    if shape_failures:
        receipt = {"native": str(native), "candidate": str(candidate), "stats": {}, "failures": shape_failures, "unchecked": ["numeric comparison not run after shape/membership failure"]}
        Path(destination).write_text(json.dumps(receipt, indent=2) + "\n")
        return receipt

    def check(identity, got, want, gate, category, structural=False):
        got, want = decimal(got), decimal(want)
        s = stats.setdefault(category, {"count": 0, "max_absolute": "0", "max_ratio": "0"})
        s["count"] += 1
        if not got.is_finite() or not want.is_finite():
            equal = ((got.is_nan() and want.is_nan()) or got == want) and got.is_signed() == want.is_signed()
            if not equal:
                failures.append({"identity": identity, "got": str(got), "reference": str(want), "kind": "class"})
            return
        if got == 0 and want == 0 and got.is_signed() != want.is_signed():
            failures.append({"identity": identity, "got": str(got), "reference": str(want), "kind": "zero_sign"})
            return
        if structural and want == 0:
            gate = ZERO
        delta = got - want
        error = abs(delta)
        ratio = error / gate if gate else (ZERO if error == 0 else D("Infinity"))
        s["max_absolute"] = str(max(D(s["max_absolute"]), error))
        if ratio > D(s["max_ratio"]):
            s.update(max_ratio=str(ratio), worst_identity=identity, worst_signed_difference=str(delta))
        if ratio > 1:
            failures.append({"identity": identity, "got": str(got), "reference": str(want), "difference": str(delta), "gate": str(gate)})

    def at(it, stage, p, row, key):
        return ref[it, stage, p, row, key]

    check("cycle count", len(result["cycles"]), max(k[0] for k in ref), ZERO, "identity")
    for cycle in result["cycles"]:
        it = cycle["iteration"]
        for stage in ("prefit", "postfit"):
            for p, pulsar in enumerate(cycle[stage]["pulsars"]):
                for i, obs in enumerate(pulsar["observations"]):
                    check([it, stage, p, i, "canonical_index"], obs["canonical_index"], i, ZERO, "identity")
                    for key, field, gate in (("residual", "residual_s", D("1e-12")), ("phase", "phase_turn", D("1e-10")), ("bbat", "bbat_mjd", D("1e-12") / 86400)):
                        check([it, stage, p, i, key], obs[field], at(it, stage, p, i, key), gate, stage + "." + key)
                    for key, field in (("pulse_number","pulse_number"),("nphase","nphase_turn"),("deleted","deleted")):
                        if (it,stage,p,i,key) in ref:
                            check([it,stage,p,i,key],obs[field],at(it,stage,p,i,key),ZERO,stage+".state")
                    if (it,stage,p,i,"sat") in ref:
                        # 0007 gives continuous SAT/BAT/BBAT the 1 ps epoch gate;
                        # input bytes/order and within-engine SAT preservation are exact.
                        check([it,stage,p,i,"sat"],obs['sat_mjd'],at(it,stage,p,i,'sat'),D('1e-12')/86400,stage+'.sat')
                    for key in ("frequency","uncertainty"):
                        if (it,stage,p,i,key) in ref:
                            check([it,stage,p,i,key],D.from_float(float(obs[key]["value"])),D.from_float(float(at(it,stage,p,i,key))),ZERO,stage+".state")
        for system in cycle["systems"]:
            cols, rows = system["columns"], system["rows"]
            np = len(cols)
            psrs = [w["pulsar"] for w in system["whitening"]]
            first = psrs[0]
            owner = -1 if any(c["pulsar"] == -1 for c in cols) else first
            check([it, owner, "solver columns"], np, at(it, "fit", first, -1, "total"), ZERO, "identity")
            for j, col in enumerate(cols):
                for field, key in (("pulsar", "owner"), ("parameter_index", "label"), ("subindex", "counter")):
                    check([it, owner, j, field], col[field], at(it, "fit", first, j, key), ZERO, "identity")
            local_columns = {}
            for p in psrs:
                nlocal = int(at(it, "prepared", p, -1, "nparams"))
                local_columns[p] = {(int(at(it, "column", p, j, "label")), int(at(it, "column", p, j, "counter"))): j for j in range(nlocal)}
                selected = [r for r in rows if r["pulsar"] == p]
                check([it, p, "rows"], len(selected), at(it, "prepared", p, -1, "ndata"), ZERO, "identity")
            reference_matrix = {"matrix": [], "white_matrix": []}
            positions = {p: 0 for p in psrs}
            white_y = [at(it, "prepared", r["pulsar"], j, "white_y") for p in psrs for j, r in enumerate([r for r in rows if r["pulsar"] == p])]
            white_scale = max(map(abs, white_y))
            for i, row in enumerate(rows):
                p = row["pulsar"]
                j = positions[p]
                positions[p] += 1
                check([it, p, j, "observation"], row["observation"], at(it, "prepared", p, j, "index"), ZERO, "identity")
                for field, key, gate in (("x_days", "x", D.from_float(4 * math.ulp(float(at(it, "prepared", p, j, "x"))))), ("residual_s", "y", D("1e-12")), ("sigma_s", "raw_sigma", ZERO)):
                    got, want = row[field], at(it, "prepared", p, j, key)
                    if key == "raw_sigma":
                        # Both decimal exporters round-trip the binary64 value;
                        # different printed decimal lengths are not changed bits.
                        got, want = D.from_float(float(got)), D.from_float(float(want))
                    check([it, p, j, key], got, want, gate, "row." + key, structural=True)
                check([it, p, j, "white_y"], system["white_data"][i], white_y[i], D("5e-14") * white_scale, "white_data", structural=True)
                check([it, p, j, "data"], system["data"][i], at(it, "prepared", p, j, "y"), D("1e-12"), "data", structural=True)
                for field in reference_matrix:
                    reference_matrix[field].append([at(it, "prepared", p, j, field + "." + str(local_columns[p][c["parameter_index"], c["subindex"]])) if c["pulsar"] in (-1, p) else ZERO for c in cols])
            scales = {}
            for field, expected in reference_matrix.items():
                scales[field] = [max(abs(row[j]) for row in expected) for j in range(np)]
                for i, row in enumerate(expected):
                    for j, value in enumerate(row):
                        check([it, owner, field, i, j], system[field][i][j], value, D("5e-14") * scales[field][j], field, structural=True)
            ci = {p: 0 for p in psrs}
            expected_constraints, expected_targets = [], []
            for i, metadata in enumerate(system["constraint_rows"]):
                p = metadata["pulsar"]
                j = ci[p]
                ci[p] += 1
                for field, key in (("label", "label"), ("subindex", "counter")):
                    check([it, p, j, "constraint." + key], metadata[field], at(it, "constraint", p, j, key), ZERO, "identity")
                expected_targets.append(at(it, "constraint", p, j, "target"))
                expected_constraints.append([at(it, "constraint", p, j, "matrix." + str(local_columns[p][c["parameter_index"], c["subindex"]])) if c["pulsar"] in (-1, p) else ZERO for c in cols])
            for p in psrs:
                check([it, p, "constraints"], ci[p], at(it, "prepared", p, -1, "nconstraints"), ZERO, "identity")
            for i, row in enumerate(expected_constraints):
                check([it, owner, i, "target"], system["constraint_targets"][i], expected_targets[i], D("5e-14") * max(map(abs, expected_targets)), "constraint_target", structural=True)
                for j, value in enumerate(row):
                    check([it, owner, i, j, "constraint"], system["constraints"][i][j], value, D("5e-14") * max(abs(r[j]) for r in expected_constraints), "constraint_matrix", structural=True)
            sol = system["solver"]
            check([it, owner, "dof"], sol["dof"], at(it, "fit", first, -1, "dof"), ZERO, "identity")
            # Older exports omitted controls. Use the stricter unweighted floor
            # for those snapshots; never select a gate from candidate controls.
            mode = ref.get((it, "prepared", first, -1, "fit_mode"), 0)
            chi = at(it, "fit", first, -1, "chi")
            check([it, owner, "chi"], sol["chi_square"], chi, D("1e-6") * abs(chi) + D("1e-12" if mode == 1 else "1e-24"), "chi_square")
            rank = 0
            for j, scale in enumerate(scales["matrix"]):
                w = at(it, "svd", owner, j, "raw")
                keep = int(at(it, "svd", owner, j, "retained"))
                rank += keep
                check([it, owner, j, "retained"], int(sol["retained"][j]), keep, ZERO, "raw_mode_identity")
                check([it, owner, j, "singular_value"], sol["singular_values"][j], w, D("1e-10") * abs(w), "raw_singular_value", structural=True)
                for field, key, relative in (("increments", "increment", D("1e-11")), ("errors", "error", D("1e-6"))):
                    expected = at(it, "fit", first, j, key)
                    check([it, owner, j, key], decimal(sol[field][j]) * scale, expected * scale, D("1e-11") + relative * abs(expected * scale), field, structural=scale == 0 or (key == "error" and expected == 0))
                for k in range(np):
                    scale_cov = (at(it, "fit", first, j, "covariance." + str(j)) * at(it, "fit", first, k, "covariance." + str(k))).sqrt()
                    check([it, owner, j, k, "covariance"], sol["covariance"][j][k], at(it, "fit", first, j, "covariance." + str(k)), D("1e-10") * scale_cov, "covariance")
            check([it, owner, "rank"], sol["rank"], rank, ZERO, "identity")
            if (it, "solver_details", owner, -1, "solves") in ref:
                def solver_details(actual, stage, identity, multiplier):
                    expected = at(it, multiplier, owner, -1, "multiplier")
                    check(identity + ["error_multiplier"], actual["error_multiplier"], expected, D("5e-14") * abs(expected), "error_multiplier", structural=True)
                    for j, value in enumerate(actual["errors_unscaled"]):
                        expected = at(it, stage, owner, j, "unscaled_error")
                        check(identity + [j, "unscaled_error"], value, expected, D("1e-10") * abs(expected), "errors_unscaled", structural=True)
                    for j, value in enumerate(actual["linearized_s"]):
                        check(identity + [j, "linearized"], value, at(it, stage, owner, j, "linearized"), D("1e-12"), "linearized_residual", structural=True)
                last = "solve." + str(int(at(it, "solver_details", owner, -1, "solves")))
                solver_details(sol, last, [it, owner], "solver_details")
                for h, actual in enumerate(sol["robust_history"]):
                    stage = "robust." + str(h)
                    inner = actual["solver"]
                    inner_stage = "solve." + str(h + 1)
                    identity = [it, owner, "robust", h]
                    solver_details(inner, inner_stage, identity, inner_stage)
                    for key in ("scale", "next_scale", "phi", "dphi"):
                        if actual[key] is not None:
                            expected = at(it, stage, owner, -1, key)
                            check(identity + [key], actual[key], expected, D("5e-14") * max(D(1), abs(expected)), "robust." + key, structural=True)
                    for key in ("white_data", "residual"):
                        scale = max(abs(at(it, stage, owner, i, key)) for i in range(len(actual[key])))
                        for i, value in enumerate(actual[key]):
                            check(identity + [key, i], value, at(it, stage, owner, i, key), D("5e-14") * scale, "robust." + key, structural=True)
                    raw = [at(it, inner_stage, owner, j, "raw") for j in range(np)]
                    mask = [v != 0 and not v < D.from_float(1e-27) * max(raw) for v in raw]
                    check(identity + ["rank"], inner["rank"], sum(mask), ZERO, "identity")
                    check(identity + ["dof"], inner["dof"], sol["dof"], ZERO, "identity")
                    expected = at(it, inner_stage, owner, -1, "chi")
                    check(identity + ["chi"], inner["chi_square"], expected, D("1e-6") * abs(expected) + D("1e-12" if mode == 1 else "1e-24"), "robust.chi")
                    for j, scale in enumerate(scales["matrix"]):
                        check(identity + [j, "retained"], int(inner["retained"][j]), int(mask[j]), ZERO, "identity")
                        check(identity + [j, "raw"], inner["singular_values"][j], raw[j], D("1e-10") * abs(raw[j]), "robust.raw", structural=True)
                        column_scale = max(abs(at(it, stage, owner, i, "matrix." + str(j))) for i in range(len(actual["white_matrix"])))
                        for i, row in enumerate(actual["white_matrix"]):
                            check(identity + [i, j, "matrix"], row[j], at(it, stage, owner, i, "matrix." + str(j)), D("5e-14") * column_scale, "robust.matrix", structural=True)
                        for field, key, relative in (("increments", "increment", D("1e-11")), ("errors", "error", D("1e-6"))):
                            expected = at(it, stage, owner, j, key)
                            check(identity + [j, key], decimal(inner[field][j]) * scale, expected * scale, D("1e-11") + relative * abs(expected * scale), "robust." + field, structural=key == "error" and expected == 0)
                        for k in range(np):
                            norm = (at(it, stage, owner, j, "covariance." + str(j)) * at(it, stage, owner, k, "covariance." + str(k))).sqrt()
                            check(identity + [j, k, "covariance"], inner["covariance"][j][k], at(it, stage, owner, j, "covariance." + str(k)), D("1e-10") * norm, "robust.covariance")
            for w in system["whitening"]:
                p = w["pulsar"]
                for j, sigma in enumerate(w["sigma"]):
                    check([it, p, j, "effective_sigma"], D.from_float(float(sigma)), D.from_float(float(at(it, "prepared", p, j, "sigma"))), ZERO, "effective_sigma")
                want = at(it, "fit", p, -1, "detL")
                check([it, p, "detL"], w["log_determinant"], want, D("5e-14") * max(D(1), abs(want)), "log_determinant")
                if ref.get((it, "prepared", p, -1, "full_covariance")) == 1:
                    n = len(w["sigma"])
                    for j in range(n):
                        lower_scale = max(abs(at(it, "whitening", p, i, "lower." + str(j))) for i in range(n))
                        for i in range(n):
                            norm = (at(it, "whitening", p, i, "covariance." + str(i)) * at(it, "whitening", p, j, "covariance." + str(j))).sqrt()
                            check([it, p, i, j, "input_covariance"], w["covariance"][i][j], at(it, "whitening", p, i, "covariance." + str(j)), D("5e-14") * norm, "input_covariance", structural=True)
                            check([it, p, i, j, "lower"], w["lower"][i][j], at(it, "whitening", p, i, "lower." + str(j)), D("5e-14") * lower_scale, "cholesky", structural=True)
            for row in system["noise"]:
                p, i = row["pulsar"], row["observation"]
                for field in ("red", "red_error", "dm", "dm_error", "chrom", "chrom_error", "solar_wind", "solar_wind_error"):
                    key = field.replace("solar_wind", "sw")
                    check([it, p, i, field], row[field], at(it, "noise", p, i, key), D("1e-12"), "noise." + field, structural=True)
            for update in system["updates"]:
                col = update["column"]
                j = next(j for j, c in enumerate(cols) if c == col)
                for field, key in (("increment", "increment"), ("fit_error", "error")):
                    scale = scales["matrix"][j]
                    expected = at(it, "fit", first, j, key)
                    relative = D("1e-11" if field == "increment" else "1e-6")
                    check([it, update["pulsar"], j, "update." + field], decimal(update[field]) * scale, expected * scale, D("1e-11") + relative * abs(expected * scale), "update." + field, structural=scale == 0 or (key == "error" and expected == 0))
                if PARAMETERS[col["parameter_index"]] != "param_f":
                    p, l, k = update["pulsar"], col["parameter_index"], col["subindex"]
                    gain = ref.get((it, "after_update", p, l, "unit_gain." + str(k)), ZERO)
                    if gain == 0:
                        if any(update[f] is not None for f in ("before_value", "after_value", "before_error", "after_error")):
                            unchecked_physical.add(PARAMETERS[l])
                        continue
                    # Convert physical-coordinate error back to source fit units,
                    # then apply the existing column-weighted seconds budget.
                    scale = scales["matrix"][j] / abs(gain)
                    binary64 = ref.get((it, "after_update", p, l, "binary64_storage." + str(k)), ZERO) == 1
                    coordinate = (lambda v: D.from_float(float(decimal(v)))) if binary64 else decimal
                    before = ref.get((it, "before_update", p, l, "value." + str(k)))
                    after = at(it, "after_update", p, l, "value." + str(k))
                    before = coordinate(before) if before is not None else None
                    after = coordinate(after)
                    change = after - before if before is not None else after
                    for when in ("before", "after", "postfit"):
                        for field in ("value", "error"):
                            if when == "postfit" and (it, "postfit_update", p, l, "defined_" + field + "." + str(k)) not in ref: continue
                            got = update[when + "_" + field]
                            if got is None: continue
                            want = coordinate(at(it, when + "_update", p, l, field + "." + str(k)))
                            physical_change = (want - before if before is not None else want) if when == "postfit" else change
                            gate = D("1e-11") + (D("1e-10") * abs(physical_change * scale) if field == "value" else D("1e-6") * abs(want * scale))
                            check([it, p, l, k, when + "_" + field], coordinate(got) * scale, want * scale, gate, "physical." + when + "_" + field, structural=field == "error")
                    continue
                p, l, k = update["pulsar"], col["parameter_index"], col["subindex"]
                f0 = at(it, "prefit", p, l, "value.0")
                t = max(abs(D(r["x_days"]) * 86400) for r in rows if r["pulsar"] == p)
                scale = t ** (k + 1) / (D(math.factorial(k + 1)) * abs(f0))
                before = at(it, "before_update", p, l, "value." + str(k))
                after = at(it, "after_update", p, l, "value." + str(k))
                check([it, p, k, "before_value"], decimal(update["before_value"]) * scale, before * scale, D("1e-11") + D("1e-10") * abs((after - before) * scale), "physical_spin_before")
                before_error = at(it, "before_update", p, l, "error." + str(k))
                check([it, p, k, "before_error"], decimal(update["before_error"]) * scale, before_error * scale, D("1e-11") + D("1e-6") * abs(before_error * scale), "physical_spin_before_error", structural=True)
                check([it, p, k, "physical_value"], decimal(update["after_value"]) * scale, after * scale, D("1e-11") + D("1e-10") * abs((after - before) * scale), "physical_spin")
                error = at(it, "after_update", p, l, "error." + str(k))
                check([it, p, k, "physical_error"], decimal(update["after_error"]) * scale, error * scale, D("1e-11") + D("1e-6") * abs(error * scale), "physical_spin_error", structural=True)
                if (it, "postfit_update", p, l, "value." + str(k)) in ref:
                    final = at(it, "postfit_update", p, l, "value." + str(k))
                    check([it, p, k, "postfit_value"], decimal(update["postfit_value"]) * scale, final * scale, D("1e-11") + D("1e-10") * abs((final - before) * scale), "physical_spin_postfit")
                    error = at(it, "postfit_update", p, l, "error." + str(k))
                    check([it, p, k, "postfit_error"], decimal(update["postfit_error"]) * scale, error * scale, D("1e-11") + D("1e-6") * abs(error * scale), "physical_spin_postfit_error", structural=True)
    receipt = {"native": str(native), "candidate": str(candidate), "stats": stats, "failures": failures,
               "unchecked": (["physical unit maps: " + ", ".join(sorted(unchecked_physical))] if unchecked_physical else [])
               + ([] if any(k[1] == "solver_details" for k in ref) else ["robust history and solver unscaled errors/multiplier/linearized residuals: legacy native trace"])
               + ([] if any(k[4] == "full_covariance" for k in ref) else ["covariance/Cholesky: legacy native trace"])
               + ["sampled covariance kernels (standalone checks)", "controls not exported by this native harness revision", "optional files (separate file checks)", "truth recovery"]}
    Path(destination).write_text(json.dumps(receipt, indent=2) + "\n")
    return receipt
