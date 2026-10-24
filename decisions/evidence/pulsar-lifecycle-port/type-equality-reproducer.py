"""Demonstrate T's pre-correction nested JSON type-equality gap without edits."""
from pathlib import Path
import copy,gzip,json,sys
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/pulsar_lifecycle_port'
sys.path.insert(0,str(HERE));import run
path=ROOT/'validation/pulsar_lifecycle/expected/matrix_1_1.json.gz'
expected=run.project_expected(json.loads(gzip.decompress(path.read_bytes())))
rows=[]
for label,transform in [('count_boolean',lambda r:r.update(count=True)),('count_float',lambda r:r.update(count=1.0)),('run_count_boolean',lambda r:r['runs'][0].__setitem__(0,True))]:
 actual=copy.deepcopy(expected)
 found=next((e,r) for e in actual for r in e['state'] if type(r.get('count')) is int and r['count']==1 and r.get('runs') and r['runs'][0][0]==1)
 event,record=found;transform(record)
 try:run.compare(actual,expected)
 except AssertionError as e:outcome='REJECTED';detail=str(e)
 else:outcome='ACCEPTED';detail='Python equality hid a changed nested JSON scalar type'
 rows.append(dict(mutation=label,phase=event['phase'],field=record['path'],outcome=outcome,detail=detail))
print(json.dumps(dict(runner_sha256=run.sha(HERE/'run.py'),case_sha256=run.sha(path),witnesses=rows),indent=2))
