#!/usr/bin/env python3
"""Recompare immutable AD public executions; preserve original comparison receipts."""
import json
import check_public as c

B = c.HERE/'build'
summary = []
for profile in ['debug','release']:
    dest = B/f'recomparison-{profile}'
    dest.mkdir(parents=True, exist_ok=True)
    for path in sorted((B/f'public-{profile}').glob('*/comparison.json')):
        original = json.loads(path.read_text())
        output = path.parent/'rust.json'
        native = path.parent/'native.tsv'
        assert c.native.sha(output)==original['rust_output_sha256']
        assert c.native.sha(native)==original['native_output_sha256']
        assert not original['errors'] and original['native_returncode']==original['rust_returncode']==0
        errors, measurements = c.compare(native.read_text(), json.loads(output.read_text()))
        result = dict(original_comparison_sha256=c.native.sha(path),
                      native_output_sha256=c.native.sha(native),
                      rust_output_sha256=c.native.sha(output),
                      comparator_sha256=c.native.sha(c.HERE/'check_public.py'),
                      rechecker_sha256=c.native.sha(c.HERE/'recompare_public.py'),
                      pipelines_rerun=False, errors=errors, measurements=measurements)
        (dest/path.parent.name).with_suffix('.json').write_text(json.dumps(result, indent=2)+'\n')
        assert not errors, (path, errors)
        summary.append(dict(profile=profile, case=path.parent.name, errors=errors))
assert len(summary)==260
(B/'recomparison-summary.json').write_text(json.dumps(summary, indent=2)+'\n')
print('260 stored public outputs pass the corrected averaged schema/unit comparator; no pipelines rerun')
