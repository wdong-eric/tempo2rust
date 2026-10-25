"""Source sscanf constraint grammar through naturally loaded public fitting."""
import argparse
from run_public import HERE, ROOT, run


def cases():
    out = HERE / "build/constraint-payloads/inputs"
    out.mkdir(parents=True, exist_ok=True)
    base = (HERE / "inputs/parameter-prior.par").read_text()
    result = []
    for name, error in [("prior-hex", float(1e-10).hex()),
                        ("prior-prefix", "1e-10suffix"),
                        ("prior-trailing", "1e-10 ignored trailing text")]:
        par = out / (name + ".par")
        par.write_text(base.replace("10.12345678905 1e-10", "10.12345678905 " + error))
        result.append((name, [par], {}))
    base = (ROOT / "validation/input_to_bat/fixtures/chromx.par").read_text()
    base = base.replace("NE_SW_IFUNC 1\n", "NE_SW_IFUNC 1 1\n")
    tim = out / "solar.tim"
    tim.write_text("FORMAT 1\n" + "".join(
        f"solar{i} {700 + 700 * (i % 3)} {54999 + i / 4} 1 pks\n" for i in range(24)))
    for name, payload in [
        ("solar-sigma", "NE_SW_IFUNC_SIGMA 0x1p-1suffix trailing text"),
        ("solar-sin", "NE_SW_IFUNC_SIN 0x1p0 0x1p-2 55000 0x1p8 0x1p-1suffix trailing text"),
    ]:
        par = out / (name + ".par")
        par.write_text(base + "\nMODE 1\nCONSTRAIN " + payload + "\n")
        result.append((name, [par], {"tims": [str(tim)]}))
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", choices=["debug", "release"], default="debug")
    run(parser.parse_args().profile, cases=cases(), directory="constraint-payloads")
