"""Freeze all selected definitions and source conditions before native execution."""
import re
from profile import HERE, PRIMARY, blob, sha, save
ROUTINES = {
 'ifteph.C': [('IFTswap4',69,75),('IFTswapInt',77,81),('IFTswapInts',83,89),
 ('IFTswap8',91,99),('IFTswapDouble',101,105),('IFTswap8N',107,120),
 ('IFTswapDoubles',122,126),('IFTE_init',128,195),('IFTE_close_file',197,201),
 ('IFTE_get_Vals',204,284),('IFTE_get_DeltaT_DeltaTDot',287,294),
 ('IFTE_DeltaT',296,301),('IFTE_DeltaTDot',303,308),('IFTE_get_vE_vEDot',310,323),
 ('IFTE_get_vE',325,329),('IFTE_get_vEDot',331,335),('IFTEinterp',388,476)],
 'tt2tdb.C': [('tt2tb',65,194),('init_ifte',197,208),('IF_deltaT',210,215),('FB_deltaT',217,308)],
 'read_fortran.h': [('open_file',35,65),('close_file',66,69),('read_character',71,76),
 ('read_char',77,80),('read_int',82,103),('read_float',105,125),('read_double',127,148),('read_record_int',149,154)]}
def inventory():
 result=[]
 for f in PRIMARY:
  data=blob(f); lines=data.decode().splitlines()
  # Preserve line numbers while dropping comments.
  clean=re.sub(r'/\*.*?\*/|//[^\n]*',lambda m:'\n'*m[0].count('\n'),data.decode(),flags=re.S).splitlines()
  for name,a,b in ROUTINES[f]:
   assert re.search(r'\b'+name+r'\s*\(', '\n'.join(lines[a-1:b]))
   result.append(dict(file=f,name=name,start=a,end=b,sha256=sha(data),
       conditions=[dict(line=i,text=clean[i-1].strip()) for i in range(a,b+1)
                   if re.search(r'\b(if|for|while)\s*\(',clean[i-1])]))
 assert len(result)==29
 return dict(routines=result,contract_sha256=sha((HERE/'SOURCE_MAP.md').read_bytes()))
if __name__=='__main__':
 save(HERE/'outputs/inventory.json',inventory())
 print('Frozen 29 routine definitions and source conditions')
