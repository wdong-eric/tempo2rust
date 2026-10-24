"""Bounded compiled witnesses for U's expression-specific O2 contractions."""
import hashlib,json,subprocess
from pathlib import Path
from freeze import ROOT,HERE
OUT=HERE/'build/arithmetic';U=ROOT/'validation/terrestrial_resources';CC='/opt/homebrew/bin/g++-16'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 OUT.mkdir(parents=True,exist_ok=True)
 mapping=json.loads((U/'outputs/frozen/source_map.json').read_text())
 selected=['fang','ITRF_to_GRS80','GRS80_to_ITRF','NMF_hydrostatic','NMF_wet']
 text='#include <cmath>\n#include <cstdio>\n#include "tempo2.h"\n#define GRS80_A 6378137.0\n#define GRS80_F 1.0/298.257222101\n'
 text+='\n'.join(('' if r['name']=='fang' else 'void\n' if r['name'] in ['ITRF_to_GRS80','GRS80_to_ITRF'] else 'double\n')+'\n'.join(r['source']) for r in mapping['routines'] if r['name'] in selected)
 text+='''
void report(const char* name, double value) { printf("%s %a\\n",name,value); }
int main() {
  for (int mode=0;mode<=4;mode++)report("fang",fang(mode,123456.789));
  observatory o={};o.x=0;o.y=-4500000;o.z=-4500000;ITRF_to_GRS80(&o);
  report("latitude",o.latitude_grs80);report("height",o.height_grs80);
  o.latitude_grs80=-60*M_PI/180;o.longitude_grs80=-M_PI;o.height_grs80=-500;
  GRS80_to_ITRF(&o);report("forward-z",o.z);ITRF_to_GRS80(&o);report("roundtrip-h",o.height_grs80);
  for (double latitude: {22.5,37.5,52.5,60.0,67.5}) {
    report("hydrostatic",NMF_hydrostatic(53580.625,latitude*M_PI/180,100,5*M_PI/180));
    report("wet",NMF_wet(latitude*M_PI/180,.1));
  }
  double a=.123456789,b=.987654321,f=.314159265;
  report("eop-final",a+f*(b-a));
  double pressure=100.12345,latitude=.6,height=415,mh=6.373595076695443,wet=.1,mw=6.537036088606678;
  double zhd=.02268*pressure/(299792458.0*(1.0-.00266*cos(latitude)-2.8e-7*height));
  report("hydrostatic-zenith",zhd);report("driver-final",zhd*mh+wet*mw);
}
'''
 path=OUT/'bounded.cc';path.write_text(text)
 src=U/'build/ordinary/source';flags=['-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src)]
 commands=[];values=[]
 for mode,extra in [('source',[]),('unfused',['-ffp-contract=off'])]:
  exe=OUT/mode;cmd=[CC,*flags,*extra,str(path),'-o',str(exe)];subprocess.run(cmd,check=True,capture_output=True)
  output=subprocess.check_output([str(exe)]);(OUT/(mode+'.txt')).write_bytes(output);commands.append(dict(command=cmd,executable_sha256=sha(exe),output_sha256=sha(OUT/(mode+'.txt'))));values.append(output.decode().splitlines())
 differences=[dict(index=i,source=a,unfused=b) for i,(a,b) in enumerate(zip(*values)) if a!=b];assert differences
 for file in ['observatory','eop','tropo']:
  cmd=[CC,*flags,'-g','-fverbose-asm','-S',str(src/(file+'.C')),'-o',str(OUT/(file+'.s'))];subprocess.run(cmd,check=True,capture_output=True);commands.append(dict(command=cmd,assembly_sha256=sha(OUT/(file+'.s'))))
 result=dict(status='PASS',pin=mapping['pin'],raw_source_hashes=mapping['hashes'],compiler_sha256=sha(Path(CC).resolve()),compiler=subprocess.check_output([CC,'--version'],text=True),bounded_source_sha256=sha(path),commands=commands,differences=differences)
 (OUT/'receipt.json').write_text(json.dumps(result,indent=2)+'\n');print('Arithmetic witnesses:',len(differences),'bounded changed scalars; exact source assembly retained')
if __name__=='__main__':main()
