#!/usr/bin/env python3
"""Collect this 0109-0111 local handoff and retain the complete original Linux ledger."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import zipfile
from common import ROOT, sha


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--linux-archive',type=Path,required=True)
    p.add_argument('--out',type=Path,required=True)
    a=p.parse_args();out=a.out.resolve();out.mkdir(parents=True,exist_ok=False)
    with zipfile.ZipFile(a.linux_archive) as z:
        raw=z.read('manifest.json')
        assert hashlib.sha256(raw).hexdigest()=='6484c6b3238d807478740c4a8f20ae3770c0a99ae0b7481bdb72307ca140dd15'
        manifest=json.loads(raw)['files'];raw=z.read('linux/receipt.json')
        assert hashlib.sha256(raw).hexdigest()==manifest['linux/receipt.json']['sha256']
        baseline=json.loads(raw);assert len(baseline['stages'])==67
        assert sum(row['exit']==0 for row in baseline['stages'].values())==54
        (out/'original-linux-receipt.json').write_bytes(raw)
    build=ROOT/'validation/release/build'
    selected={name:build/name for name in [
        'third-followthrough-v1','third-prebootstrap-v1',
        'third-candidate-v1','third-cache-candidate-v1','third-lmst-candidate-v1','third-final-candidate-v1','third-final-candidate-v2',
        'third-native-portable-v1','third-native-tim-v1','third-native-propagation-v2','third-native-observer-v1',
        'third-n-v1','third-j-v1','third-portable-v1','third-aa-v1','third-ab-v1','third-y-v2','third-w-v2',
        'third-cache-default-v1-build','third-cache-combined-v1-build','third-cache-combined-v2-build',
        'third-cache-default-check-v1','third-cache-default-check-v2','third-cache-combined-check-v1',
        'third-spin-profile-v1','third-spin-profile-v2','third-event-profile-v1',
        'third-event-trace-v1','third-event-trace-v2','third-contraction-profile-v1','third-contraction-profile-v2',
        'third-output-v1','third-output-v2',
    ]}
    selected.update({'clock-native-short':Path('/tmp/al-clock-n-20261101'),
        'clock-comparison-short':Path('/tmp/al-clock-r-20261101'),
        'terrestrial-native-short':Path('/tmp/t2-local/linux/native-corpora'),
        'terrestrial-comparison-short':Path('/tmp/t2-local/linux/corpus')})
    evidence={}
    for label,directory in selected.items():
        assert directory.is_dir(),directory
        subprocess.run([sys.executable,ROOT/'validation/release/collect_evidence.py',
            '--input',directory,'--out',out/label],check=True)
        evidence[label]={'original_directory':str(directory),'manifest':sha(out/label/'manifest.json')}
        receipt=directory/'receipt.json'
        if receipt.exists():
            data=json.loads(receipt.read_text())
            if data.get('status')=='running':
                # These recorded exceptions predate the successful successor.
                # Preserve their raw receipts; do not relabel them as successes.
                failed_attempts={'third-event-trace-v1':'event-trace.stderr',
                    'third-contraction-profile-v1':'contraction-profile.stderr'}
                failure=build/'third-followthrough-v1'/failed_attempts[label]
                assert 'Traceback (most recent call last)' in failure.read_text()
                evidence[label]['aborted_attempt_traceback']=sha(failure)
            evidence[label]['receipt']=sha(receipt)
            evidence[label]['status']=data.get('status')
            if 'stages' in data:evidence[label]['stages']=data['stages']
    record={'scope':'Complete frozen Linux ledger plus explicitly separate focused macOS evidence; no inferred corrected Linux execution',
        'original_linux_stages':baseline['stages'],'original_linux_receipt':manifest['linux/receipt.json'],
        'original_linux_archive':{'path':str(a.linux_archive.resolve()),'sha256':sha(a.linux_archive)},
        'focused_evidence':evidence,'recipe':sha(__file__),
        'source_revision':subprocess.check_output(['git','-C',ROOT,'rev-parse','HEAD'],text=True).strip(),
        'corrected_linux_execution':'pending central review and dispatch',
        'unimplemented_arithmetic':['spin pending Linux measurements','chromatic','cosmic-string','secular','unverified contractions'],
        'qualification':'pending'}
    (out/'ledger.json').write_text(json.dumps(record,indent=2)+'\n')
    print('67 original Linux stages and',len(evidence),'separately bound local evidence groups',flush=True)
