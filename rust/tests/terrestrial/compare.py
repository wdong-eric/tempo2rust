#!/usr/bin/env python3
"""Decision 0004 terrestrial projection, independent of barycentric-v1."""
from decimal import Decimal, localcontext
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parent
CONTRACT = json.loads((ROOT / "contract.json").read_text())
MANIFEST = json.loads((ROOT / "manifest.json").read_text())


def golden(fixture):
    path = ROOT / "outputs" / (fixture + ".json")
    if hashlib.sha256(path.read_bytes()).hexdigest() != MANIFEST["fixtures"][fixture]:
        raise ValueError("immutable A2 fixture hash mismatch: " + fixture)
    return json.loads(path.read_text())


def field_spec(key):
    if re.fullmatch(r"clock_step_s_\d+", key):
        return CONTRACT["fields"]["clock_step_s_*"]
    if re.fullmatch(r"clock_to_\d+", key):
        return CONTRACT["fields"]["clock_to_*"]
    return CONTRACT["fields"][key]


def projection(data, reference=False):
    fixture = data["fixture"]
    if fixture not in CONTRACT["stages_by_fixture"] or data["status"] != "completed":
        raise ValueError("unsupported or incomplete terrestrial fixture")
    if not reference and (data.get("comparison_profile") != "terrestrial-v1" or data.get("schema_version") != 1):
        raise ValueError("terrestrial-v1 schema 1 required")
    parameters = {}
    for key, value in data["parameters"].items():
        if key in CONTRACT["omittable_legacy_parameters"]:
            if value["set"] != 0 or value["fit_flag"] != 0 or Decimal(value["value"]) != 0:
                raise ValueError("unsupported supplied parameter: " + key)
        elif key in CONTRACT["parameters"]:
            parameters[key] = {k: value[k] for k in ("set", "fit_flag", "value")}
        else:
            raise ValueError("unknown parameter: " + key)
    if set(parameters) != set(CONTRACT["parameters"]):
        raise ValueError("missing parameter")
    if set(data["stages"]) != set(CONTRACT["stages_by_fixture"][fixture]):
        raise ValueError("missing or unexpected stage")
    stages = {}
    for stage, rows in data["stages"].items():
        if len(rows) != len(data["original_observations"]):
            raise ValueError("stage count differs: " + stage)
        fields = CONTRACT["parsed_fields"][:]
        if stage != "parsed":
            fields += CONTRACT["bats_fields"]
        if stage == "residuals":
            fields += CONTRACT["residual_fields"]
        stages[stage] = []
        for row in rows:
            if any(row[k] != v for k, v in {"site": "pks", "deleted": "0", "clockCorr": "1", "delayCorr": "1"}.items()):
                raise ValueError("unsupported site/flags")
            clock = [k for k in row if re.fullmatch(r"clock_(?:step_s|to)_\d+", k)]
            if stage == "parsed" and clock:
                raise ValueError("clock values before calculation")
            count = len(clock) // 2
            wanted = {f"clock_{key}_{i}" for i in range(count) for key in ("step_s", "to")}
            if set(clock) != wanted or set(row) != set(fields) | wanted:
                raise ValueError("missing or unsupported stage field")
            stages[stage].append(row)
    if reference:
        resources = {key.removeprefix("$RUNTIME/"): value["sha256"] for key, value in data["resources"]["files"].items()
                     if key.startswith("$RUNTIME/")}
    else:
        resources = {r["path"]: r["sha256"] for r in data["resources"]}
        if len(resources) != len(data["resources"]):
            raise ValueError("duplicate resource identity")
        # Unused indexed clocks and local paths are not cross-engine equality fields.
        for r in data["resources"]:
            if r["role"] == "applied clock" and r["path"] not in CONTRACT["required_resources"]:
                raise ValueError("unexpected applied clock: " + r["path"])
    resources = {key: resources[key] for key in CONTRACT["required_resources"]}
    return dict(fixture=fixture, configuration=data["configuration"],
                parameters=parameters, original_observations=data["original_observations"],
                resources=resources, stages=stages)


def compare(expected, actual):
    if expected != golden(expected["fixture"]):
        raise ValueError("expected object differs from immutable A2 golden")
    expected, actual = projection(expected, True), projection(actual)
    errors, measurements = [], []
    for key in ("fixture", "configuration", "original_observations", "resources"):
        if expected[key] != actual[key]:
            errors.append(f"{key}: exact mismatch")
    with localcontext() as context:
        context.prec = 90
        for key, value in expected["parameters"].items():
            new = actual["parameters"][key]
            if any(value[k] != new[k] for k in ("set", "fit_flag")):
                errors.append(f"{key}: flag mismatch")
            a, b = Decimal(new["value"]), Decimal(value["value"])
            if not a.is_finite() or not b.is_finite() or abs(a-b) > abs(b)*Decimal("1e-30"):
                errors.append(f"{key}: parameter mismatch")
        for stage, rows in expected["stages"].items():
            others = actual["stages"][stage]
            if len(rows) != len(others):
                errors.append(f"{stage}: count mismatch")
                continue
            for i, (row, new) in enumerate(zip(rows, others)):
                if set(row) != set(new):
                    errors.append(f"{stage}[{i}]: clock count/fields mismatch")
                    continue
                for key, value in row.items():
                    spec = field_spec(key)
                    if spec["comparison"] == "exact":
                        if value != new[key]:
                            errors.append(f"{stage}[{i}].{key}: exact mismatch")
                        continue
                    a, b = Decimal(new[key]), Decimal(value)
                    if not a.is_finite() or not b.is_finite():
                        errors.append(f"{stage}[{i}].{key}: nonfinite")
                        continue
                    difference = (a-b)*Decimal(spec.get("difference_scale", "1"))
                    measurement = dict(stage=stage, toa=i, field=key, signed_difference=str(difference),
                                       unit=spec["comparison_unit"], gate=spec["absolute_tolerance"])
                    measurements.append(measurement)
                    if abs(difference) > Decimal(spec["absolute_tolerance"]):
                        errors.append(f"{stage}[{i}].{key}: signed difference {difference} {spec['comparison_unit']}")
    return errors, measurements


def response_diagnostic(data):
    errors, rows = [], []
    with localcontext() as context:
        context.prec = 90
        stages = data["stages"]
        for i, row in enumerate(stages["bats"]):
            derivative = (Decimal(stages["slope_plus"][i]["bat_mjd"]) -
                          Decimal(stages["slope_minus"][i]["bat_mjd"]))*86400/Decimal("0.002")
            response = (Decimal(stages["offset_4ns"][i]["bat_mjd"])-Decimal(row["bat_mjd"]))*86400
            difference = response-Decimal("4e-9")*derivative
            rows.append(dict(toa=i, local_bat_derivative=str(derivative), response_s=str(response),
                             signed_error_s=str(difference), gate_s="1e-12"))
            if abs(difference) > Decimal("1e-12"):
                errors.append(f"4ns response[{i}]: {difference} s")
    return errors, rows


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("actual", type=Path)
    args = parser.parse_args()
    data = json.loads(args.actual.read_text())
    try:
        errors, _ = compare(golden(data["fixture"]), data)
        if data["fixture"] == "terrestrial_4ns":
            errors += response_diagnostic(data)[0]
    except (KeyError, ValueError) as error:
        errors = [str(error)]
    print("\n".join(errors) if errors else "PASS")
    raise SystemExit(bool(errors))
