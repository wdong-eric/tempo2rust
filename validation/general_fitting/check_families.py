"""Public family fits from unchanged AD/AC models plus explicit fit flags.

This is a fidelity check, not truth recovery. Authored recipes expand cadence
to give vector families positive degrees of freedom, without seeding fit buffers.
"""
from pathlib import Path
from decimal import Decimal
import argparse, json
from run_public import ROOT, HERE, run, sha

AD = ROOT / "validation/timing_models/fixtures"
AC = ROOT / "validation/input_to_bat/fixtures"


def cases():
    out = HERE / "build/families/inputs"
    out.mkdir(parents=True, exist_ok=True)
    tim = out / "cadence.tim"
    tim.write_text("FORMAT 1\n" + "".join(
        f"family{i} {[700, 1400, 2100, 800, 1600][i % 5]} {Decimal('53997.25') + Decimal(i) / 4} {1 + i % 4} @ -f {'A' if i % 3 == 0 else 'B'}\n"
        for i in range(36)))
    recipes = [
        ("event-amplitudes", AD / "events.par", ["EXPPH_1", "GAUSAMP_1"], {}),
        *[("glitch-" + key.lower(), AD / "glitches.par", [key], {}) for key in
          ["GLPH_1", "GLF0_1", "GLF1_1", "GLF2_1", "GLF0D_1", "GLF0D2_1", "GLF0D3_1"]],
        ("jumps-fd", AD / "jumps-fd.par", ["FD1", "FD2"], {"JUMP": "JUMP -f A 0.0000012 1", "FD1JUMP": "FD1JUMP -f B 1.5e-6 1"}),
        *[(name, AD / (name + ".par"), ["SIFUNC"], {}) for name in ["ifunc-linear", "ifunc-step", "ifunc-sinc"]],
        ("waves", AD / "waves.par", ["WAVE_OM", "WAVDM_OM"], {}),
        ("wave-dm-scale", AD / "wave-dm-scale.par", ["WAVE_OM"], {}),
        ("quad-interpolation", AD / "quad-interpolation.par", ["SQIFUNC_p", "SQIFUNC_c"], {}),
        ("quadrupole", AD / "quadrupole.par", ["QUAD_OM"], {}),
        ("gw-single", AD / "gw-single.par", ["GW_SINGLE"], {}),
        ("gw-memory", AD / "gw-memory.par", ["GWM_AMP"], {}),
        ("gw-memory-polarizations", AD / "gw-memory-polarizations.par", ["GWM_A1", "GWM_A2"], {}),
        ("gw-cosmic-string", AD / "gw-cosmic-string.par", ["GWCS_A1", "GWCS_A2"], {}),
        *[("shapelet-" + kind, AD / "shapelet.par", [], {"TNShapeletEvent": f"TNShapeletEvent 3 54000.5 2 {scale} 1e-5 -2e-6 3e-6 1"}) for kind, scale in [("red", 0), ("dm", 2)]],
        *[("ell1h-" + key.lower(), AD / "ell1h-h3.par", [key], {}) for key in ["A1", "EPS1", "EPS2", "TASC", "H3", "PB"]],
    ]
    # Retain the unconstrained cases above. These separate neighbors remove the
    # known ZERO/interpolation degeneracy through source-supported constraints.
    additions = {}
    for name in ["ifunc-linear", "ifunc-step", "ifunc-sinc"]:
        recipes.append((name + "-auto", AD / (name + ".par"), ["SIFUNC"], {}))
        additions[name + "-auto"] = ["CONSTRAIN AUTO"]
    recipes.append(("quad-plus-auto", AD / "quad-interpolation.par", ["SQIFUNC_p"], {}))
    additions["quad-plus-auto"] = ["CONSTRAIN AUTO"]
    # Multiple sky positions separate the two GW polarizations in a global fit.
    for name, flags in [("gw-single", ["GW_SINGLE"]), ("quadrupole", ["QUAD_OM"]), ("gw-memory-polarizations", ["GWM_A1", "GWM_A2"]), ("gw-cosmic-string", ["GWCS_A1", "GWCS_A2"])]:
        recipes.append((name + "-global", AD / (name + ".par"), flags, {}))
    ac_tim = out / "dispersion.tim"
    ac_tim.write_text("FORMAT 1\n" + "".join(
        f"dispersion{i} {[700, 1400, 2100][i % 3]} {Decimal('54999.25') + Decimal(i) / 4} {1 + i % 4} pks\n"
        for i in range(28)))
    selected_tims = {}
    for key in ["DM", "DM1", "CM", "CM1", "DMX_0001", "CHROMX_0001", "NE_SW", "NE_SW_SIN", "NE_SW_IFUNC"]:
        name = "dispersion-" + key.lower()
        recipes.append((name, AC / "chromx.par", [key], {}));selected_tims[name] = ac_tim
    for name, flags, extra in [
        ("dm-poly", ["DM", "DM1"], ["TNDMAmp -14", "TNDMGam 4", "TNDMC 2", "TNsubtractPoly 1"]),
        ("chrom-poly", ["CM", "CM1"], ["TNChromAmp -14", "TNChromGam 4", "TNChromC 2", "TNsubtractPoly 1"]),
        ("solar-gp-fit", ["NE_SW"], ["SWGPAmp -14", "SWGPGam 4", "SWGPC 2", "TNsubtractSWGP 1"]),
        ("solar-gp-fixed", [], ["SWGPAmp -14", "SWGPGam 4", "SWGPC 2", "TNsubtractSWGP 1"]),
        # Keep the first-residual unwritten-signal cases above. Disabling only
        # residualtn subtraction still exercises the fitted SWGP mean transfer.
        ("solar-gp-fit-no-subtraction", ["NE_SW"], ["SWGPAmp -14", "SWGPGam 4", "SWGPC 2"]),
        ("solar-gp-fixed-no-subtraction", [], ["SWGPAmp -14", "SWGPGam 4", "SWGPC 2"]),
    ]:
        recipes.append((name, AC / "chromx.par", flags, {}))
        additions[name] = extra;selected_tims[name] = ac_tim
    base = ROOT / "validation/fitting/inputs/f0.par"
    for key in ["RAJ", "DECJ"]:
        name = "position-" + key.lower()
        recipes.append((name, base, [key], {"F0": "F0 10.12345678902 0 1e-40"}))
        selected_tims[name] = ROOT / "validation/fitting/inputs/pks.tim"
    epochs = out / "epochs.tim"
    epochs.write_text("FORMAT 1\n" + "".join(
        f"epoch{i} {[700, 1400, 2100][i % 3]} {Decimal('53997.25') + Decimal(i // 3) / 2 + Decimal(i % 3) / 345600} {1 + i % 4} @ -f {'A' if i // 3 % 2 == 0 else 'B'} -tobs {1200 + i * 60}\n"
        for i in range(30)))
    for name, extra in [
        ("group-noise", ["TNGroupNoise -f A -14 4 2"]),
        ("ecorr", ["TNECORR -f A 1.5"]),
        ("secorr", ["TNSECORR -f B 1.5"]),
        ("ecorr-secorr", ["TNECORR -f A 1.5", "TNSECORR -f B 1.5"]),
        ("red-spectral", ["TNRedAmp -14", "TNRedGam 4", "TNRedC 2", "TNRedFLow -0.3", "TNRedCorner 2"]),
    ]:
        recipes.append((name, base, [], {}));additions[name] = extra;selected_tims[name] = epochs
    result, records = [], []
    for name, source, fit, replacement in recipes:
        lines, seen = [], set()
        for line in source.read_text().splitlines():
            key = line.split()[0]
            if key in replacement:
                line = replacement[key];seen.add(key)
            if key in fit:
                assert len(line.split()) == 2, (name, line)
                line += " 2" if name.endswith("-global") else " 1";seen.add(key)
            lines.append(line)
        assert seen == set(fit) | set(replacement), (name, seen)
        par = out / (name + ".par")
        lines.extend(additions.get(name, []))
        par.write_text("\n".join(lines) + "\nMODE 1\nNITS 2\n")
        pars = [par]
        if name.endswith("-global"):
            second = out / (name + "-second.par")
            second.write_text(par.read_text().replace("PSRJ AD_PHASE", "PSRJ AE_SECOND").replace("RAJ 01:02:03.123456789", "RAJ 06:02:03.123456789").replace("DECJ -20:30:40.123456789", "DECJ -45:30:40.123456789"))
            pars.append(second)
        selected_tim = selected_tims.get(name, tim)
        result.append((name, pars, {"tims": [str(selected_tim)] * len(pars)}))
        records.append({"case": name, "source": str(source), "source_sha256": sha(source), "fit_flags": fit,
                        "replacements": replacement, "additions": additions.get(name, []), "par_sha256": {str(p): sha(p) for p in pars}, "tim_sha256": sha(selected_tim)})
    (out / "recipes.json").write_text(json.dumps({"builder_sha256": sha(__file__), "cases": records}, indent=2) + "\n")
    return result


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("--profile", choices=["debug", "release"], default="debug")
    p.add_argument("cases", nargs="*")
    args = p.parse_args()
    run(args.profile, args.cases, cases(), "families")
