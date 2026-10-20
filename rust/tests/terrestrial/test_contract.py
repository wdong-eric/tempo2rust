#!/usr/bin/env python3
"""Mutation checks of the declared comparison gates, not timing implementation."""
from copy import deepcopy
from decimal import Decimal, localcontext
import json
from pathlib import Path
import sys
import compare


def check(actual):
    fixture = actual["fixture"]
    expected = compare.golden(fixture)

    def accepted(data):
        try:
            return not compare.compare(expected, data)[0]
        except (KeyError, ValueError):
            return False

    assert accepted(actual)
    checks = []
    # Each numeric quantity gets independent inside/outside-budget probes.
    with localcontext() as context:
        context.prec = 90
        for key in actual["stages"]["residuals"][0]:
            spec = compare.field_spec(key)
            if spec["comparison"] == "exact":
                continue
            for factor, verdict in [("0.5", True), ("2", False)]:
                changed = deepcopy(actual)
                changed["stages"]["residuals"][0][key] = str(
                    Decimal(expected["stages"]["residuals"][0][key])
                    + Decimal(factor)*Decimal(spec["absolute_tolerance"])/Decimal(spec.get("difference_scale", "1")))
                assert accepted(changed) == verdict, (key, factor)
                checks.append(f"{key}: {factor} budget")
        for key in ("nphase_turn", "pulse_number", "clock_to_0", "site", "deleted", "clockCorr", "delayCorr"):
            changed = deepcopy(actual)
            changed["stages"]["residuals"][0][key] = "altered"
            assert not accepted(changed), key
            checks.append(f"exact {key}")
        for field in ("value", "set", "fit_flag"):
            changed = deepcopy(actual)
            changed["parameters"]["DM"][field] = "101" if field == "value" else 1-actual["parameters"]["DM"][field]
            assert not accepted(changed)
            checks.append(f"parameter {field}")
    for name in ("configuration", "resource", "missing_resource", "missing_stage", "missing_field",
                 "missing_clock", "extra_stage", "count", "original_token", "order", "profile", "nonfinite"):
        changed = deepcopy(actual)
        if name == "configuration":
            changed["configuration"]["ipm"] = "0"
        elif name == "resource":
            next(r for r in changed["resources"] if r["path"] == "ephemeris/DE405.1950.2050")["sha256"] = "wrong"
        elif name == "missing_resource":
            changed["resources"] = [r for r in changed["resources"] if r["path"] != "ephemeris/DE405.1950.2050"]
        elif name == "missing_stage":
            del changed["stages"]["bats"]
        elif name == "missing_field":
            del changed["stages"]["bats"][0]["roemer_s"]
        elif name == "missing_clock":
            del changed["stages"]["bats"][0]["clock_to_0"]
        elif name == "extra_stage":
            changed["stages"]["fabricated"] = changed["stages"]["bats"]
        elif name == "count":
            changed["stages"]["bats"].pop()
        elif name == "original_token":
            changed["original_observations"][0]["sat_mjd_text"] += "0"
        elif name == "order":
            changed["original_observations"].reverse()
        elif name == "profile":
            changed["comparison_profile"] = "barycentric-v1"
        elif name == "nonfinite":
            changed["stages"]["residuals"][0]["residual_s"] = "NaN"
        assert not accepted(changed), name
        checks.append(name)
    return checks


if __name__ == "__main__":
    actual = json.loads(Path(sys.argv[1]).read_text())
    checks = check(actual)
    print(json.dumps({"status": "PASS", "mutation_checks": len(checks), "checks": checks}, indent=2))
