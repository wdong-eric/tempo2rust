"""Fresh source witness for helpers and partially seeded embedded links."""
import argparse,copy,gzip,importlib.util,json,secrets,shlex,shutil,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
from run import ROOT,HERE,OUT,ENV,sha,save,build,identity,compare,protected

def inputs():
 q=json.load(gzip.open(ROOT/'validation/par_specialized/observations.json.gz'))[0]
 assert q['id']=='initial_and_helpers'
 helpers=[[int(a[1],0),*map(int,a[2:])] for a in q['actions']]
 helpers += [[v,r,d,c] for v in [0,(1<<64)-1,0x12345678abcdef00] for r,d,c in [(-2147483648,2147483647,0),(0,1,2),(3,4,-1)]]
 return dict(helpers=helpers,shifts=[[v,i] for v in [0,1,(1<<64)-1,0x0102030405060708,1<<63] for i in range(64)],clamps=[-2147483648,-1,0,1,2,3,4,2147483647]),q

def native_build(profile,mutation=None):
 spec=importlib.util.spec_from_file_location('s_profile',ROOT/'validation/pulsar_lifecycle/profile.py');s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s)
 dest=OUT/profile/'native'/('ordinary' if mutation is None else mutation);s.BUILD=dest;src=s.prepare()
 if mutation:
  p=src/'initialise.C';text=p.read_text();needle='psr->param[i].nLinkTo   = 0;';assert text.count(needle)==1
  field='linkTo[2]' if mutation=='reset_link_to' else 'linkFrom[4]';text=text.replace(needle,needle+f' psr->param[i].{field} = 0;');p.write_text(text)
 cc=Path(shutil.which('g++-16')).resolve();exe=dest/'probe';cmd=[str(cc),'-O0' if profile=='debug' else '-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src),str(HERE/'supplement.cc'),*[str(src/u) for u in s.UNITS],'-lquadmath','-o',str(exe)]
 p=subprocess.run(cmd,env=ENV,capture_output=True,text=True);save(dest/'build.json',dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr));assert p.returncode==0,p.stderr
 headers={}
 for u in [HERE/'supplement.cc',*[src/u for u in s.UNITS]]:
  dep=subprocess.check_output([str(cc),'-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src),'-M',str(u)],text=True,env=ENV)
  headers.update({p:sha(Path(p)) for p in shlex.split(dep.split(':',1)[1].replace('\\\n','')) if Path(p).is_file()})
 backend=Path(subprocess.check_output([str(cc),'-print-prog-name=cc1plus'],text=True).strip()).resolve()
 runtime={}
 for n in ['libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib']:
  p=Path(subprocess.check_output([str(cc),'-print-file-name='+n],text=True).strip()).resolve();runtime[str(p)]=sha(p)
 ident=dict(executable=str(exe),executable_sha256=sha(exe),compiler=str(cc),compiler_sha256=sha(cc),backend=str(backend),backend_sha256=sha(backend),dependencies=headers,runtime=runtime,runner=sha(Path(__file__)),harness=sha(HERE/'supplement.cc'),source_helpers=sha(ROOT/'validation/pulsar_lifecycle/profile.py'),mutation=mutation)
 save(dest/'identity.json',ident);return exe,ident

def validate(ident):
 for k in ['executable','compiler','backend']:assert sha(Path(ident[k]))==ident[k+'_sha256'],k
 for kind in ['dependencies','runtime']:assert {p:sha(Path(p)) for p in ident[kind]}==ident[kind]
 assert sha(Path(__file__))==ident['runner'] and sha(HERE/'supplement.cc')==ident['harness']

def native_run(exe,ident,data):
 dest=exe.parent;token=secrets.token_hex(16);text=''
 for key in ['helpers','shifts','clamps']:
  rows=data[key];text+=str(len(rows))+'\n'+'\n'.join(' '.join(map(str,row)) if isinstance(row,list) else str(row) for row in rows)+'\n'
 inp=dest/'input.txt';inp.write_text(text);ih=sha(inp);out=dest/'result.json';out.unlink(missing_ok=True)
 validate(ident);p=subprocess.run([str(exe),str(inp),str(out),token],env=ENV,capture_output=True,text=True,timeout=120);assert p.returncode==0,(p.stdout,p.stderr)
 raw=json.loads(out.read_text());assert raw.pop('nonce')==token and sha(inp)==ih;validate(ident)
 save(dest/'launch.json',dict(input_sha256=ih,output_sha256=sha(out),nonce=token,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,identity=ident));return raw

def main():
 p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],required=True);a=p.parse_args();protected();data,q=inputs();exe=build(a.profile);ident=identity(exe)
 dest=OUT/a.profile/'helpers';dest.mkdir(parents=True,exist_ok=True);token=secrets.token_hex(16);inp=dest/'input.json';save(inp,data|dict(nonce=token));ih=sha(inp);out=dest/'rust.json';out.unlink(missing_ok=True)
 proc=subprocess.run([str(exe),'pulsar_lifecycle_tests::helpers_probe','--exact','--ignored','--nocapture'],env=dict(ENV,TEMPO2_T_INPUT=str(inp),TEMPO2_T_OUTPUT=str(out)),capture_output=True,text=True,timeout=120)
 assert proc.returncode==0,(proc.stdout,proc.stderr);assert identity(exe)==ident and sha(inp)==ih
 rust=json.loads(out.read_text());assert rust.pop('nonce')==token and rust.pop('input_sha256')==ih
 nexe,nid=native_build(a.profile);native=native_run(nexe,nid,data);compare(rust,native)
 compare(rust['helpers'][:9],[e['result'] for e in q['events'] if e['phase']=='end'])
 initial=json.load(gzip.open(ROOT/'validation/par_specialized/initial_state.json.gz'))['p0.TNsubtractPoly'];b=bytearray(initial['size'])
 for offset,h in initial['nonzero']:b[offset:offset+len(bytes.fromhex(h))]=bytes.fromhex(h)
 compare(rust['initial'],int.from_bytes(b,'little'))
 mutations=[]
 for name in ['reset_link_to','reset_link_from']:
  e,i=native_build(a.profile,name);r=native_run(e,i,data)
  assert r['partial_links_allocate']!=native['partial_links_allocate'] and r['partial_links_initialise']!=native['partial_links_initialise']
  mutations.append(dict(name=name,allocation=r['partial_links_allocate'],initialization=r['partial_links_initialise'],identity=i))
 assert identity(exe)==ident;protected();save(dest/'receipt.json',dict(status='PASS',identity=ident,native=nid,mutations=mutations,helpers=len(data['helpers']),Q_helpers=9,shifts=len(data['shifts']),clamps=len(data['clamps']),input_sha256=ih,output_sha256=sha(out),command_stdout=proc.stdout,command_stderr=proc.stderr))
 print(a.profile,'Q + supplemental helpers/links and actual native mutation witnesses PASS',flush=True)
if __name__=='__main__':main()
