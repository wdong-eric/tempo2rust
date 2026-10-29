import hashlib, json, os, pathlib, shutil, subprocess, tempfile
ROOT=pathlib.Path('/Users/Eric.Dong/Downloads/tempo2rust')
CANDIDATE='8d9c11afdf56160d909952aa23f98d9478db71a9'
BASELINE='6acaf53d2e13291eb6d0b0988faf7dee7d33a635'
OUT=pathlib.Path(tempfile.mkdtemp(prefix='tempo2-al-shallow-fixed-'))
pathlib.Path('/tmp/tempo2-al-shallow-fixed-current').write_text(str(OUT)+'\n')
shutil.copyfile(__file__,OUT/'executed-script.py')
print(OUT,flush=True)
checkout=OUT/'checkout';checkout.mkdir();commands=[]
env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1')
def run(label,args,expected=0,cwd=checkout):
    args=[str(a) for a in args]
    p=subprocess.run(args,cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    (OUT/(label+'.stdout')).write_bytes(p.stdout);(OUT/(label+'.stderr')).write_bytes(p.stderr)
    commands.append({'label':label,'argv':args,'cwd':str(cwd),'status':p.returncode})
    (OUT/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
    if expected is not None:assert p.returncode==expected,(label,p.returncode)
    return p
run('init',['git','init'])
run('origin',['git','remote','add','origin',ROOT])
run('candidate-fetch',['git','fetch','--depth=1','origin',CANDIDATE])
run('candidate-checkout',['git','checkout','--detach','FETCH_HEAD'])
missing=run('missing-baseline',['python3','-B',checkout/'validation/release/source_audit.py','--out',OUT/'missing','--native-repository',ROOT/'tempo2'],expected=None)
receipt=json.loads((OUT/'missing/receipt.json').read_text())
assert missing.returncode != 0 and receipt['status']=='failure' and receipt['baseline_status']=='unavailable'
assert 'changed_rust_bodies' not in receipt
run('baseline-fetch',['git','fetch','--no-tags','--depth=1','origin',BASELINE])
head=run('head-after-fetch',['git','rev-parse','HEAD']).stdout.decode().strip();assert head==CANDIDATE
assert run('shallow',['git','rev-parse','--is-shallow-repository']).stdout.strip()==b'true'
run('available-baseline',['python3','-B',checkout/'validation/release/source_audit.py','--out',OUT/'available','--native-repository',ROOT/'tempo2'])
available=json.loads((OUT/'available/receipt.json').read_text())
assert available['status']=='pass' and available['baseline_status']=='available'
assert len(available['unchanged_rust_bodies'])==155
assert available['changed_rust_bodies']==available['added_rust_bodies']==available['removed_rust_bodies']==[]
run('focused-regression',['python3','-B','-m','unittest','test_source_audit'],cwd=checkout/'validation/release')
assert not run('checkout-status',['git','status','--porcelain']).stdout
summary={'candidate':CANDIDATE,'baseline':BASELINE,'missing_baseline_exit':missing.returncode,'missing_baseline_status':receipt['status'],'available_baseline_status':available['status'],'unchanged_rust_files':155,'changed_rust_files':0,'focused_regression':'pass','head_after_fetch':head,'shallow':True,'scope':'Focused source-audit correction; no scientific output or Linux execution claimed','script_sha256':hashlib.sha256((OUT/'executed-script.py').read_bytes()).hexdigest(),'commands_sha256':hashlib.sha256((OUT/'commands.json').read_bytes()).hexdigest()}
(OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2),flush=True)
