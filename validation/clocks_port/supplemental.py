#!/usr/bin/env python3
"""0029: safe new glob/index/observatory inputs; frozen J evidence stays untouched."""
import argparse
import copy
import hashlib
import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path
import run as port

PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
PATTERNS=['a','*','?','[ab]','[!x]','[^x]','[[:alpha:]]','[=a=]','[[=a=]]','[[.a.]]',r'\.a','.*','[.]a',r'back\\slash','a/','a//','./a','a/../b']


def native_helper(work):
    blob=subprocess.check_output(['git','-C',str(port.ROOT/'tempo2'),'show',PIN+':clkcorr.C'])
    body=blob[blob.index(b'static std::vector<std::string> globClockFiles('):blob.index(b'static void ensureClockCorrectionsInitialized(int dispWarnings)')]
    assert hashlib.sha256(body).hexdigest()=='0fbb59b932cf2ce934917d89e4307e1aedc353ecc108c75a439a1b85110b1543'
    source=work/'glob.cc';exe=work/'glob-native'
    source.write_bytes(b'#include <glob.h>\n#include <cstdio>\n#include <cstdlib>\n#include <string>\n#include <vector>\n#include <algorithm>\n'+body+b'\nint main(int argc,char**argv){if(argc!=2)return 2;for(const auto &p:globClockFiles(argv[1])){for(unsigned char c:p)std::printf("%02x",c);std::puts("");}}\n')
    compiler=shutil.which('g++-16');assert compiler
    command=[compiler,'-O2','-std=gnu++11',str(source),'-o',str(exe)]
    subprocess.run(command,check=True,capture_output=True,env=port.ENV)
    return exe,dict(source_pin=PIN,source_blob_sha256=hashlib.sha256(blob).hexdigest(),function_sha256=hashlib.sha256(body).hexdigest(),harness_sha256=port.sha(source),harness=source.read_text(),executable_sha256=port.sha(exe),compiler_sha256=port.sha(Path(compiler)),compiler=subprocess.check_output([compiler,'--version'],text=True).splitlines()[0],command=command)


def integrated_cases():
    route=dict(id='glob_route_order',classification='ordinary',files={
        'roots/a/clock/edge.clk':'# A B 1\n0 10\n10 10\n',
        'roots/a b/clock/edge.clk':'# A B 1\n0 20\n10 20\n'},
        actions=[['profile'],['debug','1','0'],['env','{case}/roots/*',''],['obs','0','5','T','1','0'],['clock','0','A','B','0'],['tt','0']])
    observatory=dict(id='glob_observatory_order',classification='ordinary',files={
        'roots/*/observatory/observatories.dat':'6378137 0 0 Base Z BASE\n',
        'roots/*/observatory/aliases':'',
        'roots/a/observatory/site.dat':'6378137 0 0 Short T SHORT\n',
        'roots/a b/observatory/site.dat':'6378137 0 0 Space T SPACE\n',
        'roots/a/clock/edge.clk':'# SHORT B 1\n0 10\n10 10\n',
        'roots/a b/clock/edge.clk':'# SPACE B 1\n0 20\n10 20\n'},
        actions=[['profile'],['debug','1','0'],['env','{case}/roots/*',''],['obs','0','5','T','1','0'],['clock','0','@null','B','0'],['tt','0']])
    separators=dict(id='glob_root_separators',classification='ordinary',files={
        'root/observatory/observatories.dat':'6378137 0 0 Site T A\n',
        'root/observatory/aliases':'',
        'root/clock/edge.clk':'# A B 1\n0 20\n10 20\n'},
        actions=[['profile'],['debug','1','0'],['env','{case}/root//',''],['obs','0','5','T','1','0'],['clock','0','@null','B','0'],['tt','0']])
    for case in [route,observatory,separators]:case['last_action']=len(case['actions'])-1
    return [route,observatory,separators]


def native_case(case,base):
    # port.execute has already materialized these same immutable files. J's
    # accepted binary writes only fresh temporary action/event artifacts here.
    work=base/'cases'/case['id']
    actions=[[x.replace('{case}',str(work)) for x in a] for a in case['actions']]
    inp=work/'native-actions.tsv';out=work/'native-events.jsonl';out.unlink(missing_ok=True)
    inp.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in actions)+'\n')
    exe=port.J/'build/probe';before=port.sha(exe)
    assert before==port.load(port.J/'outputs/manifest.json')['native']['executable_sha256']
    command=[str(exe),str(inp),str(out)]
    p=subprocess.run(command,capture_output=True,text=True,timeout=20,env=port.ENV)
    assert p.returncode==0 and out.is_file(),(case['id'],p.stderr)
    def norm(s):return s.replace(str(base),'{build}').replace(str(port.J/'build'),'{build}')
    raw=dict(events=[json.loads(norm(s)) for s in out.read_text().splitlines()],stdout=norm(p.stdout),stderr=norm(p.stderr),returncode=p.returncode)
    assert port.sha(exe)==before
    return port.comparison_record(case,raw),dict(command=command,executable_sha256=before,input_sha256=port.sha(inp),output_sha256=port.sha(out),returncode=p.returncode)


def reject(changed,expected):
    try:port.compare(changed,expected)
    except AssertionError:return 'rejected'
    raise AssertionError('mutation escaped comparison')


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--profile',choices=['debug','release'],required=True);args=parser.parse_args()
    sources=port.executed_sources();frozen=port.frozen_inputs();exe=port.build(args.profile)
    helper=[];integrated=[];identities=[];mutations={}
    with tempfile.TemporaryDirectory(prefix='clockG-',dir='/private/tmp') as tmp:
        native,compiler=native_helper(Path(tmp))
        base=Path(tmp)/('x'*(len(str(port.J/'build'))-len(tmp)-1));base.mkdir()
        tree=base/'tree';tree.mkdir()
        for name in ['a','b','x','^','!','-','.', '.a','ab','a b',r'back\slash','colon:']:
            d=tree/name;d.mkdir(exist_ok=True);(d/'a.clk').write_text('');(d/'.hidden.clk').write_text('')
        # All eighteen original review fixtures are unchanged, including the
        # absent parent a.clk that controls whether .* includes ../a.clk.
        for index,suffix in enumerate(PATTERNS):
            pattern=str(tree)+'/'+suffix+'/*.clk'
            case=dict(id=f'glob_helper_{index}',classification='ordinary',actions=[['glob',pattern]],files={},last_action=0)
            raw,identity=port.execute(case,exe,base);identities.append(identity)
            native_paths=subprocess.check_output([str(native),pattern],env=port.ENV,text=True).splitlines()
            actual=raw['events'][-1]['result'];expected=dict(paths_hex=native_paths)
            counts=port.compare(actual,expected)
            portable=lambda paths:[bytes.fromhex(p).replace(str(base).encode(),b'{build}').hex() for p in paths]
            helper.append(dict(pattern=suffix,native=portable(native_paths),rust=portable(actual['paths_hex']),counts=counts))
        for case in integrated_cases():
            raw,identity=port.execute(case,exe,base);identities.append(identity)
            expected,native_identity=native_case(case,base)
            actual=port.comparison_record(case,raw);counts=port.compare(actual,expected,case['id'])
            # The full source event comparison includes paths, index/cache
            # order, diagnostics, destination bytes and the selected value.
            assert actual['events'][-1]['result']['value']['decimal']=='20'
            integrated.append(dict(id=case['id'],case=case,reference=expected,rust=actual,counts=counts,native_identity=native_identity))
        # Same gate as the positive comparisons; reproduce each original class
        # of fault rather than merely perturbing an arbitrary number.
        rows={r['pattern']:r for r in helper}
        expected=rows['*']['native'];changed=copy.deepcopy(expected)
        i=next(i for i,p in enumerate(changed) if b'/a b/' in bytes.fromhex(p));changed[i],changed[i+1]=changed[i+1],changed[i]
        mutations['PathBuf component order']=reject(changed,expected)
        mutations['wrong membership']=reject(rows['[ab]']['native'],rows['[^x]']['native'])
        for syntax in ['[[:alpha:]]','[[=a=]]','[[.a.]]']:
            mutations[syntax]=reject(rows['a']['native'],rows[syntax]['native'])
        mutations['omitted dot entry']=reject(rows['.*']['native'][1:],rows['.*']['native'])
        mutations['collapsed separators']=reject(rows['a']['native'],rows['a//']['native'])
        for row in integrated[:2]:
            changed=copy.deepcopy(row['rust'])
            changed['events'][-1]['result']['value']={'class':'finite','bits':'4024000000000000','decimal':'10'}
            mutations[row['id']+' selected correction']=reject(changed,row['reference'])
        review_path=port.ROOT/'decisions/evidence/clocks/glob-review.json'
        review=port.load(review_path);old_rejections=[]
        for row in review['checks']:
            if row['equal']:
                port.compare(row['rust'],row['native'])
            else:
                assert reject(row['rust'],row['native'])=='rejected'
                old_rejections.append(row['pattern'])
        assert old_rejections==['*','[^x]','[[:alpha:]]','[[=a=]]','[[.a.]]','.*','a/','a//']
        assert port.sha(native)==compiler['executable_sha256']
    port.validate_identity(sources,port.executed_sources());port.validate_identity(frozen,port.frozen_inputs())
    port.save(port.OUT/(args.profile+'_glob_supplemental.json'),dict(status='PASS',profile=args.profile,helpers=helper,integrated=integrated,mutations=mutations,reviewed_delivery_rejections=old_rejections,review_evidence_sha256=port.sha(review_path),identities=identities,native=compiler,source_hashes=sources,executable_sha256=port.sha(exe),frozen_hashes=frozen))
    print(f'{args.profile}: 18 helper patterns, 3 integrated cases, 9 mutations PASS')


if __name__=='__main__':main()
