from pathlib import Path
import hashlib,json,shutil,subprocess,tempfile
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
FROZEN=Path(Path('/tmp/tempo2-al-shallow-fixed-current').read_text().strip())/'checkout'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-al-collector-review-'))
Path('/tmp/tempo2-al-collector-review-current').write_text(str(OUT)+'\n')
shutil.copyfile(__file__,OUT/'executed-script.py')
source=ROOT/'validation/observer_coordinates/outputs/reference.json.gz'
relative=Path('linux/native-corpora/staging/validation/observer_coordinates/outputs/reference.json.gz')
dest=OUT/'input'/relative;dest.parent.mkdir(parents=True);shutil.copyfile(source,dest)
(OUT/'input/receipt.json').write_text('{"fixture":"collector retains completed native corpus"}\n')
args=['python3','-B',str(FROZEN/'validation/release/collect_evidence.py'),'--input',str(OUT/'input'),'--out',str(OUT/'selected')]
p=subprocess.run(args,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
(OUT/'stdout.txt').write_bytes(p.stdout);(OUT/'stderr.txt').write_bytes(p.stderr)
manifest=json.loads((OUT/'selected/manifest.json').read_text())
summary={'reviewed_revision':'8d9c11afdf56160d909952aa23f98d9478db71a9','collector_sha256':hashlib.sha256((FROZEN/'validation/release/collect_evidence.py').read_bytes()).hexdigest(),'command':args,'exit':p.returncode,'source':str(source),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'source_bytes':source.stat().st_size,'input_relative':str(relative),'compressed_reference_selected':str(relative) in manifest['files'],'control_receipt_selected':'receipt.json' in manifest['files'],'selected_files':list(manifest['files']),'scope':'Artifact retention of real compressed reference evidence; no native or Rust scientific execution'}
assert p.returncode==0 and not summary['compressed_reference_selected'] and summary['control_receipt_selected']
(OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(OUT);print(json.dumps(summary,indent=2))
