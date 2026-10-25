"""0053 reserved-index correction with a fully defined active-slot neighbor."""
from pathlib import Path
import subprocess, hashlib, json, difflib
from fitting_corrections import corrected

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
BUILD = HERE / "build/copy";BUILD.mkdir(parents=True, exist_ok=True)
identity = json.loads((HERE / "build/native/identity.json").read_text())
old = subprocess.check_output(["git", "-C", str(ROOT / "tempo2"), "show", identity["pin"] + ":tempo2Util.C"]).decode()
new = corrected("tempo2Util.C", old)
(BUILD / "correction.patch").write_text("".join(difflib.unified_diff(old.splitlines(True), new.splitlines(True), fromfile="original/tempo2Util.C", tofile="corrected/tempo2Util.C")))
records = []
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
for name, source in [("original", old), ("corrected", new)]:
    path = BUILD / (name + ".C");path.write_text(source)
    cmd = identity["commands"][0].copy();cmd[cmd.index("-c") + 1] = str(path);cmd[cmd.index("-o") + 1] = str(BUILD / (name + ".o"))
    subprocess.run(cmd, capture_output=True, check=True);commands = [cmd]
    exe = BUILD / name
    cmd = [cmd[0], "-O2", "-std=gnu++11", "-DHAVE_CONFIG_H", "-I" + str(ROOT / "validation/reference/build/source"), str(HERE / "copy_probe.cc"), str(BUILD / (name + ".o")), str(ROOT / "validation/reference/build/libtempo2-reference.a"), "-lquadmath", "-lgfortran", "-o", str(exe)]
    subprocess.run(cmd, capture_output=True, check=True);commands.append(cmd)
    output = subprocess.check_output([str(exe)]).decode().strip()
    assert output == ("JUMP 10 11 FDJUMP 20 21" if name == "original" else "JUMP 30 11 FDJUMP 40 21")
    records.append({"name": name, "commands": commands, "output": output, "source_sha256": sha(path), "executable_sha256": sha(exe)})
(BUILD / "receipt.json").write_text(json.dumps({"pin": identity["pin"], "probe_sha256": sha(HERE / "copy_probe.cc"), "patch_sha256": sha(BUILD / "correction.patch"), "records": records}, indent=2) + "\n")
print("reserved slots remain untouched; active jump/FDJUMP neighbors unchanged")
