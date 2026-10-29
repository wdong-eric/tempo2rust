from pathlib import Path
import hashlib,json,shutil,subprocess,tempfile
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
REV='1c10ae07ebbc31259f6695a95d8bd09a8a23eecc'
ORIGINAL=Path(Path('/tmp/tempo2-al-collector-review-current').read_text().strip())
OUT=Path(tempfile.mkdtemp(prefix='tempo2-al-collector-fixed-'))
Path('/tmp/tempo2-al-collector-fixed-current').write_text(str(OUT)+'\n')
shutil.copyfile(__file__,OUT/'executed-script.py')
S=OUT/'source/validation/release';S.mkdir(parents=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
source={}
for name in ['collect_evidence.py','common.py','test_collect_evidence.py']:
 data=subprocess.check_output(['git','-C',str(ROOT),'show',REV+':validation/release/'+name])
 (S/name).write_bytes(data);source[name]=sha(S/name)
args=['python3','-B',str(S/'collect_evidence.py'),'--input',str(ORIGINAL/'input'),'--out',str(OUT/'selected')]
p=subprocess.run(args,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
(OUT/'collector.stdout').write_bytes(p.stdout);(OUT/'collector.stderr').write_bytes(p.stderr)
original=json.loads((ORIGINAL/'summary.json').read_text())
m=json.loads((OUT/'selected/manifest.json').read_text())
name=original['input_relative']
assert p.returncode==0 and set(m['files'])=={name,'receipt.json'}
assert m['files'][name]=={'sha256':original['source_sha256'],'bytes':original['source_bytes']}
assert sha(OUT/'selected'/name)==original['source_sha256']
test=['python3','-B','-m','unittest','test_collect_evidence']
t=subprocess.run(test,cwd=S,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
(OUT/'regression.stdout').write_bytes(t.stdout);(OUT/'regression.stderr').write_bytes(t.stderr)
assert t.returncode==0
summary={'candidate':REV,'frozen_sources':source,'commands':[{'argv':args,'exit':p.returncode},{'argv':test,'cwd':str(S),'exit':t.returncode}],'original_reproduction':str(ORIGINAL),'selected_files':2,'compressed_reference_bytes':original['source_bytes'],'compressed_reference_sha256':original['source_sha256'],'manifest_sha256':sha(OUT/'selected/manifest.json'),'scope':'Artifact retention and inclusion/exclusion regression only; no scientific execution','script_sha256':sha(OUT/'executed-script.py')}
(OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(OUT);print(json.dumps(summary,indent=2))
