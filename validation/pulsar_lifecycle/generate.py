"""Mechanical typed seeding/export only; never emits native function implementations."""
import json
from pathlib import Path
HERE=Path(__file__).resolve().parent

def generate():
    inv=json.loads((HERE/'inventory.json').read_text());types=inv['types'];out=[]
    # parameter has independent, dynamically sized native backing.
    for kind in types:
        if kind!='parameter':out += [f'void seed({kind}& x,int v);',f'void dump(const std::string& p,const {kind}& x);']
    for kind in ['clock_correction','observation','storePrecision','FitOutput','FitInfo','pulsar']:
        fields=types[kind];out.append(f'void seed({kind}& x,int v){{')
        for r in fields:
            n=r['name']
            if kind=='pulsar' and n=='param':continue
            out.append(f' seed(x.{n},v);')
        out.append('}')
        out.append(f'void dump(const std::string& p,const {kind}& x){{')
        for r in fields:
            if kind=='pulsar' and r['name']=='param':out.append(' params(p+".param",x,true,true);')
            else:out.append(f' dump(p+".{r["name"]}",x.{r["name"]});')
        if kind=='pulsar':out.append(' constraintPayloads(p,x);')
        out.append('}')
    out.append('void natural(const std::string& p,const pulsar& x){')
    for r in types['pulsar']:
        n=r['name'];state=r['natural_state']
        if n=='param':out.append(' params(p+".param",x,false,false);')
        elif state.startswith('unwritten'):out.append(f' unwritten(p+".{n}");')
        elif state.startswith('source writes string'):out.append(f' prefix(p+".{n}",x.{n},sizeof x.{n});')
        else:out.append(f' dump(p+".{n}",x.{n});')
    out.append('}')
    out.append('void retainedAfterInit(const std::string& p,const pulsar& x){')
    for r in types['pulsar']:
        if r['name']=='param':out.append(' params(p+".param",x,true,false);')
        else:out.append(f' dump(p+".{r["name"]}",x.{r["name"]});')
    out.append('}')
    return '\n'.join(out)+'\n'
if __name__=='__main__':(HERE/'generated.h').write_text(generate())
