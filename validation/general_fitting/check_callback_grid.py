"""Scalar derivative/update families on real timed inputs, including source exits."""
import argparse
import json
import os
import subprocess
from run_public import ROOT, HERE, RUNTIME, FIT_INPUTS, sha
from check_kernels import read_native, compare


def run(profile):
    out = HERE / "build/callback-grid" / profile
    out.mkdir(parents=True, exist_ok=True)
    ad = ROOT / "validation/timing_models/fixtures"
    base = (ad / "braking.par").read_text().replace("BRAKE 3.1", "BRAKE 3.1 1")
    recipes = [
        ("position", base, ["RAJ", "DECJ"], "PMRA 120 1\nPMDEC -80 1\nPMRA2 0.1 1\nPMDEC2 -0.2 1\nPX 6 1\nPMRV 0.01 1\nDSHK 0.2 1\n", FIT_INPUTS / "pks.tim"),
        ("events", (ad / "events.par").read_text(), ["EXPEP_1", "EXPPH_1", "EXPTAU_1", "EXPINDEX_1", "GAUSEP_1", "GAUSAMP_1", "GAUSSIG_1", "GAUSINDEX_1"], "", ad / "events.tim"),
        ("glitches", (ad / "glitches.par").read_text(), ["GLPH_1", "GLF0_1", "GLF1_1", "GLF2_1", "GLF0D_1", "GLF0D2_1", "GLF0D3_1", "GLTD_1", "GLTD2_1", "GLTD3_1"], "", ad / "glitches.tim"),
        ("glep", (ad / "glitches.par").read_text(), ["GLEP_1"], "", ad / "glitches.tim"),
        ("chromatic", base, ["DM"], "FDDC 0.001 1\nFDDI 2\nDM_S1YR 0.001 1\nDM_C1YR 0.002 1\nDMASSPLANET5 1e-10 1\n", FIT_INPUTS / "pks.tim"),
        ("fddi", base, [], "FDDC 0.001\nFDDI 2 1\n", FIT_INPUTS / "pks.tim"),
    ]
    identity = json.loads((HERE / "build/native/identity.json").read_text())
    for mode in [1, 2, -1]:
        extra = ''.join(f'STEL_D{axis} {mode} 1\n' + ''.join(f'TEL_D{axis}{j} {epoch} {value} 0\n' for j, epoch, value in [(1, 53900, 1e-6), (2, 54000, -2e-6), (3, 54200, 3e-6)]) for axis in 'XYZ')
        recipes.append((f'telescope-{mode}', base, [], extra, FIT_INPUTS / 'pks.tim'))
    recipes.append(('telescope-polynomial', base, [], 'TELEPOCH 54000\n' + ''.join(f'TEL{axis}{order if order else ""} 1e-8 1\n' for axis in 'XYZ' for order in range(4)), FIT_INPUTS / 'pks.tim'))
    recipes.append(('clock-offset', (ad / 'clock-offsets.par').read_text(), ['STEL_CLK_OFFS'], '', ad / 'clock-offsets.tim'))
    native = out / "native"
    compile_cmd = identity["commands"][-1].copy()
    compile_cmd[compile_cmd.index(str(HERE / "native_fit.cc"))] = str(HERE / "callback_grid_probe.cc")
    compile_cmd[compile_cmd.index("-o") + 1] = str(native)
    proc = subprocess.run(compile_cmd, capture_output=True)
    assert proc.returncode == 0, proc.stderr.decode()
    rust = ROOT / "rust/target" / profile / "examples/fitting_callback_grid"
    records = []
    for name, source, flags, extra, tim in recipes:
        directory = out / name
        directory.mkdir(exist_ok=True)
        par = directory / "input.par"
        par.write_text("\n".join(line + " 1" if line.split() and line.split()[0] in flags else line for line in source.splitlines()) + "\n" + extra)
        trace = directory / "native.txt"
        commands = [[str(native), str(trace), str(par), str(tim)], [str(rust), str(RUNTIME), str(par), str(tim)]]
        exits = []
        for engine, cmd in zip(["native", "rust"], commands):
            proc = subprocess.run(cmd, cwd=directory, env={**os.environ, "TEMPO2": str(RUNTIME)}, capture_output=True, timeout=120)
            (directory / (engine + ".stdout")).write_bytes(proc.stdout)
            (directory / (engine + ".stderr")).write_bytes(proc.stderr)
            exits.append(proc.returncode)
        failures = []
        boundary = name in ["fddi", "glep", "clock-offset"]
        if boundary:
            assert all(code != 0 for code in exits), exits
            native_diagnostic, rust_diagnostic = {
                "fddi": ("ERROR:Parameter not implemented", "t2FitFunc_notImplemented: source exit"),
                "glep": ("function t2FitFunc_stdGlitch", "t2FitFunc_stdGlitch"),
                "clock-offset": ("function t2FitFunc_ifunc", "t2FitFunc_ifunc: assertion"),
            }[name]
            for engine, diagnostic in [("native", native_diagnostic), ("rust", rust_diagnostic)]:
                output = (directory / (engine + ".stdout")).read_text() + (directory / (engine + ".stderr")).read_text()
                assert diagnostic in output, (name, engine, output)
        else:
            assert exits == [0, 0], (name, exits, (directory / "rust.stderr").read_text())
            ref = read_native(trace.read_text())
            failures = compare(ref, json.loads((directory / "rust.stdout").read_text()))
        records.append({"case": name, "commands": commands, "inputs": {str(p): sha(p) for p in [par, tim]}, "exits": exits, "source_boundary": boundary, "fields": 0 if boundary else len(ref), "failures": failures})
        print(name, "source exit" if boundary else f"{len(ref)} fields; {len(failures)} failures", flush=True)
    (out / "receipt.json").write_text(json.dumps({"profile": profile, "compile": compile_cmd, "builder_sha256": sha(__file__), "probe_sha256": sha(HERE / "callback_grid_probe.cc"), "rust_sha256": sha(rust), "native_sha256": sha(native), "records": records}, indent=2) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", choices=["debug", "release"], default="debug")
    run(parser.parse_args().profile)
