"""Reject omitted/extra membership, cycle substitution, types and signed zeros."""
from pathlib import Path
from copy import deepcopy
import argparse, hashlib, json
from decimal import Decimal as D
from compare import compare

HERE = Path(__file__).resolve().parent
BUILD = HERE / "build/comparator-controls"


def check(native, candidate, out=None):
    global BUILD
    BUILD = (out or BUILD) / candidate.parent.name
    BUILD.mkdir(parents=True, exist_ok=True)
    baseline = json.loads(candidate.read_text())
    assert not compare(native, candidate, BUILD / "baseline.json")["failures"]
    positives = []
    for target in ['final', 'robust']:
        x = deepcopy(baseline);count = 0
        for cycle in x['cycles']:
            for system in cycle['systems']:
                solvers = [system['solver']] if target == 'final' else [step['solver'] for step in system['solver']['robust_history']]
                for solver in solvers:
                    for key in ['singular_values', 'retained']:solver[key].reverse()
                    count += 1
        if not count:continue
        path = BUILD / (target + '-permuted.json');path.write_text(json.dumps(x) + '\n')
        result = compare(native, path, BUILD / (target + '-permuted.comparison.json'))
        assert not result['failures'], result['failures']
        assert json.loads(path.read_text()) == x
        assert any(any(a != b for a,b in row['reference_to_candidate']) for row in result['mode_permutations'])
        positives.append({'name':target+' paired permutation','mode_permutations':result['mode_permutations']})
    mutants = {}
    x = deepcopy(baseline)
    for c in x["cycles"]:
        c["systems"] = []
        c["prefit"]["pulsars"] = []
        c["postfit"]["pulsars"] = []
    mutants["empty_workflow"] = x
    x = deepcopy(baseline);x["cycles"] = [deepcopy(x["cycles"][0])] * len(x["cycles"])
    if len(baseline["cycles"]) > 1: mutants["duplicated_first_cycle"] = x
    x = deepcopy(baseline)
    for c in x["cycles"]: c["systems"][0]["noise"] = []
    mutants["missing_noise"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = "-0"
    mutants["noise_zero_sign"] = x
    for name, path in [
        ("cycle", ["cycles"]), ("system", ["cycles", 0, "systems"]),
        ("pulsar", ["cycles", 0, "prefit", "pulsars"]),
        ("observation", ["cycles", 0, "postfit", "pulsars", 0, "observations"]),
        ("noise", ["cycles", 0, "systems", 0, "noise"]),
        ("column", ["cycles", 0, "systems", 0, "columns"]),
        ("matrix_row", ["cycles", 0, "systems", 0, "matrix"]),
        ("covariance_column", ["cycles", 0, "systems", 0, "solver", "covariance", 0]),
        ("update", ["cycles", 0, "systems", 0, "updates"]),
        ("whitening", ["cycles", 0, "systems", 0, "whitening"]),
        ("unscaled_error", ["cycles", 0, "systems", 0, "solver", "errors_unscaled"]),
        ("singular_mode", ["cycles", 0, "systems", 0, "solver", "singular_values"]),
        ("retention_flag", ["cycles", 0, "systems", 0, "solver", "retained"]),
        ("linearized_residual", ["cycles", 0, "systems", 0, "solver", "linearized_s"]),
    ]:
        for mutation in ("missing", "extra"):
            x = deepcopy(baseline);a = x
            for key in path: a = a[key]
            if mutation == "missing": a.pop()
            else: a.append(deepcopy(a[-1]))
            mutants[mutation + "_" + name] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = 0
    mutants["numeric_type"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = "NaN"
    mutants["numeric_class"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["columns"][1]["parameter"] = "renamed"
    mutants["renamed_column"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][1]["column"]["parameter"] = "renamed"
    mutants["renamed_physical_update"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][1]["after_value"] = None
    mutants["missing_physical_value"] = x
    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][1]["increment"] = "1e100"
    mutants["changed_update_increment"] = x
    # New observers distinguish unwritten ZERO storage from a defined zero and
    # check its physical result in addition to the duplicate fitted increment.
    if "defined_value.0" in native.read_text():
        for field, replacement in [("before_value", "0"), ("after_value", "1e100"), ("after_error", None)]:
            x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][0][field] = replacement
            mutants["zero_physical_" + field] = x
    if "\tpostfit_update\t" in native.read_text():
        for j in (0, 1):
            for field in ("postfit_value", "postfit_error"):
                for replacement in (None, "1e100"):
                    x = deepcopy(baseline);x["cycles"][0]["systems"][0]["updates"][j][field] = replacement
                    mutants[f"postfit_{j}_{field}_{replacement}"] = x
    for key in ["errors_unscaled", "error_multiplier", "linearized_s"]:
        x = deepcopy(baseline);solver = x["cycles"][0]["systems"][0]["solver"]
        if isinstance(solver[key], list): solver[key][0] = "1e100"
        else: solver[key] = "1e100"
        mutants["changed_" + key] = x
    if baseline["cycles"][0]["systems"][0]["solver"]["robust_history"]:
        for action in ["missing", "extra"]:
            x = deepcopy(baseline);history = x["cycles"][0]["systems"][0]["solver"]["robust_history"]
            if action == "missing": history.pop()
            else: history.append(deepcopy(history[-1]))
            mutants[action + "_robust_step"] = x
        for key in ["scale", "next_scale", "white_data", "white_matrix", "residual"]:
            x = deepcopy(baseline);step = x["cycles"][0]["systems"][0]["solver"]["robust_history"][0]
            if key == "white_matrix": step[key][0][0] = "1e100"
            elif isinstance(step[key], list): step[key][0] = "1e100"
            else: step[key] = "1e100"
            mutants["robust_" + key] = x
        for key in ["errors_unscaled", "error_multiplier", "linearized_s", "singular_values", "increments", "errors", "covariance", "chi_square"]:
            x = deepcopy(baseline);solver = x["cycles"][0]["systems"][0]["solver"]["robust_history"][0]["solver"]
            if key == "covariance": solver[key][0][0] = "1e100"
            elif isinstance(solver[key], list): solver[key][0] = "1e100"
            else: solver[key] = "1e100"
            mutants["robust_solver_" + key] = x
    if baseline["cycles"][0]["systems"][0]["whitening"][0]["lower"] is not None:
        for key in ["covariance", "lower"]:
            for action in ["missing", "extra", "changed"]:
                x = deepcopy(baseline);matrix = x["cycles"][0]["systems"][0]["whitening"][0][key]
                if action == "missing": matrix.pop()
                elif action == "extra": matrix.append(deepcopy(matrix[-1]))
                else: matrix[0][0] = "1e100"
                mutants[action + "_input_" + key] = x
    for target in ['final', 'robust']:
        if target == 'robust' and not baseline['cycles'][0]['systems'][0]['solver']['robust_history']:continue
        for name in ['rank', 'retention', 'value', 'nonfinite', 'physical_covariance', 'physical_increment']:
            x = deepcopy(baseline);solver = x['cycles'][0]['systems'][0]['solver']
            if target == 'robust':solver = solver['robust_history'][0]['solver']
            if name == 'rank':solver['rank'] += 1
            elif name == 'retention':solver['retained'][0] = not solver['retained'][0]
            elif name == 'value':solver['singular_values'][0] = str(D(solver['singular_values'][0]) * D('1.1'))
            elif name == 'nonfinite':solver['singular_values'][0] = '-NaN'
            elif name == 'physical_covariance':solver['covariance'] = [list(reversed(row)) for row in reversed(solver['covariance'])]
            else:solver['increments'].reverse()
            mutants[target + '_' + name] = x
    x = deepcopy(baseline);x['cycles'][0]['postfit']['pulsars'][0]['observations'].reverse()
    mutants['physical_observation_permutation'] = x
    records = []
    for name, x in mutants.items():
        path = BUILD / (name + ".json");path.write_text(json.dumps(x) + "\n")
        result = compare(native, path, BUILD / (name + ".comparison.json"))
        assert result["failures"], name + " falsely passed"
        records.append({"name": name, "failures": len(result["failures"])})
    # Comparison-only class controls: change both independent reference and
    # candidate to a defined class, then require its opposite sign to fail.
    for native_value, same, opposite in [
        ("NaN", {"class": "nan"}, {"class": "-nan"}),
        ("-NaN", "-NaN", "NaN"),
        ("Infinity", {"class": "+infinity"}, {"class": "-infinity"}),
        ("-Infinity", "-Infinity", "Infinity"),
        ("-0", "-0", "0"),
    ]:
        name = "class_sign_" + native_value
        ref = BUILD / (name + ".tsv")
        lines = native.read_text().splitlines()
        idx = next(i for i, line in enumerate(lines) if line.startswith("1\tnoise\t0\t0\tred\t"))
        lines[idx] = "1\tnoise\t0\t0\tred\t" + native_value
        ref.write_text("\n".join(lines) + "\n")
        path = BUILD / (name + ".json")
        x = deepcopy(baseline);x["cycles"][0]["systems"][0]["noise"][0]["red"] = same
        path.write_text(json.dumps(x) + "\n")
        assert not compare(ref, path, BUILD / (name + ".same.json"))["failures"]
        x["cycles"][0]["systems"][0]["noise"][0]["red"] = opposite
        path.write_text(json.dumps(x) + "\n")
        result = compare(ref, path, BUILD / (name + ".opposite.json"))
        assert len(result["failures"]) == 1, name
        records.append({"name": name, "failures": len(result["failures"])})
    # Comparator-only counterfactuals: change the two input spectra together,
    # leaving physical fields alone, to protect discarded-value and equality gates.
    for name, small, keep in [('discarded', '1e-30', False), ('cutoff_equality', str(D.from_float(1e-27)), True), ('zero', '0', False)]:
        x = deepcopy(baseline);solver = x['cycles'][0]['systems'][0]['solver'];n = len(solver['singular_values'])
        values = [str(D(2) ** -j) for j in range(n)];values[-1] = small
        flags = [True] * n;flags[-1] = keep
        solver.update(singular_values=values, retained=flags, rank=sum(flags))
        lines = native.read_text().splitlines()
        for i,line in enumerate(lines):
            row=line.split('\t')
            if row[0]=='1' and row[1]=='svd' and row[3]!='-1':
                j=int(row[3])
                if row[4]=='raw':row[5]=values[j]
                elif row[4]=='retained':row[5]=str(int(flags[j]))
                lines[i]='\t'.join(row)
        ref = BUILD / (name + '.tsv');ref.write_text('\n'.join(lines)+'\n')
        path = BUILD / (name + '.json');path.write_text(json.dumps(x)+'\n')
        result=compare(ref,path,BUILD/(name+'.same.json'));assert not result['failures'],result['failures']
        # Move value+flag pairs together, then corrupt the tiny value or its flag.
        for key in ['singular_values','retained']:solver[key].reverse()
        path.write_text(json.dumps(x)+'\n')
        result=compare(ref,path,BUILD/(name+'.paired.json'));assert not result['failures'],result['failures']
        positives.append({'name':name+' paired','mode_permutations':result['mode_permutations']})
        if name=='discarded':
            wrong=deepcopy(x);wrong['cycles'][0]['systems'][0]['solver']['singular_values'].reverse()
            bad=BUILD/'flag-value-mismatch.json';bad.write_text(json.dumps(wrong)+'\n')
            rejection=compare(ref,bad,BUILD/'flag-value-mismatch.comparison.json')
            assert rejection['failures']
            records.append({'name':'flag/value mismatch','failures':len(rejection['failures'])})
            solver['singular_values'][0]='2e-30'
        else:solver['retained'][0]=not keep
        path.write_text(json.dumps(x)+'\n')
        result=compare(ref,path,BUILD/(name+'.changed.json'));assert result['failures'],name
        records.append({'name':name+' changed','failures':len(result['failures'])})
        if name=='discarded':assert any(row['identity'][-1]=='singular_value' for row in result['failures'])
    for native_value, opposite in [('NaN', '-NaN'), ('-NaN', 'NaN'), ('Infinity', '-Infinity')]:
        x=deepcopy(baseline);x['cycles'][0]['systems'][0]['solver']['singular_values'][0]=native_value
        lines=native.read_text().splitlines()
        for index,line in enumerate(lines):
            if line.startswith('1\tsvd\t0\t0\traw\t'):
                lines[index]='1\tsvd\t0\t0\traw\t'+native_value
        ref=BUILD/('mode-class-'+native_value+'.tsv');ref.write_text('\n'.join(lines)+'\n')
        path=BUILD/('mode-class-'+native_value+'.json');path.write_text(json.dumps(x)+'\n')
        result=compare(ref,path,BUILD/('mode-class-'+native_value+'.same.json'))
        assert not result['failures'],result['failures']
        assert result['mode_permutations'][0]['reference_to_candidate'] is None
        x['cycles'][0]['systems'][0]['solver']['singular_values'][0]=opposite
        path.write_text(json.dumps(x)+'\n')
        result=compare(ref,path,BUILD/('mode-class-'+native_value+'.opposite.json'))
        assert result['failures']
        records.append({'name':'mode class sign '+native_value,'failures':len(result['failures'])})
    sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
    (BUILD / "receipt.json").write_text(json.dumps({"native": str(native), "candidate": str(candidate), "native_sha256": sha(native), "candidate_sha256": sha(candidate), "comparator_sha256": sha(HERE / "compare.py"), "controls": records, "positive_permutations": positives, "mode_acceptance": "0120"}, indent=2) + "\n")
    print("baseline passes;", len(records), "mutants rejected")


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("--native", type=Path, default=HERE / "build/public/release/spin/native.tsv")
    p.add_argument("--candidate", type=Path, default=HERE / "build/public/release/spin/rust.json")
    p.add_argument("--out", type=Path)
    a = p.parse_args();check(a.native, a.candidate, a.out)
