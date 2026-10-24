"""Static typed seed/dump visitors; no lifecycle algorithm is generated here."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
inv=json.load(open(ROOT/'rust/verification/pulsar_lifecycle/freeze/field-and-operation-map.json'))
names={'clock_correction':'SourceClockCorrection','observation':'SourceObservation','storePrecision':'StorePrecision','FitOutput':'FitOutput','FitInfo':'FitInfo','pulsar':'SourcePulsar'}
lines=['// Generated typed test visitors; source behavior is called, never emulated.']
for k in names:
 fields=inv['types'][k];typ=names[k]
 lines+=[f'fn seed_{k}(x: &mut {typ}, v:i32, opaque:PayloadView) {{']
 for f in fields:
  n=f['name'];t=f['type'];d=len(f['extents'])
  if n=='param' and k=='pulsar':continue
  ref='x.'+n
  if t in names and not f['pointers']:
   if d:lines.append(f'    for cell in {ref}.iter_mut() {{seed_{t}(cell,v,opaque);}}')
   else:lines.append(f'    seed_{t}(&mut {ref},v,opaque);')
   continue
  if t=='char' and d and not f['pointers']:
   if d==1:lines.append(f'    seed_string(&mut {ref}[..],v);')
   else:lines.append(f'    for row in {ref}.iter_mut() {{seed_string(row,v);}}')
   continue
  v={'double':'v as f64+0.25','float':'v as f32+0.5','longdouble':'Float::with_val(128,v as f64+0.125)','int':'v','long long':'v as i64','unsigned':'v as u32','param_label':'v','constraint_label':'v','uint64_t':'v as u64','char':'v as u8','enum constraint':'if v==11 {Constraint::constraint_dmmodel_mean} else {Constraint::constraint_dmmodel_cw_0}','enum series_type':'if v==11 {SeriesType::series_undefined} else {SeriesType::series_simple_pn}','paramDerivFunc':'Some(Callback {identity:if v==11 {0} else {1},callable:deriv})','constraintDerivFunc':'Some(Callback {identity:if v==11 {0} else {1},callable:constraint})','paramUpdateFunc':'Some(Callback {identity:if v==11 {0} else {1},callable:update})'}.get(t)
  if f['pointers']:v='Some(opaque)' if t=='void' else 'None'
  assert v is not None,f
  if d:
   for i in range(d):lines.append(f'    for cell{i} in '+(ref if i==0 else 'cell'+str(i-1))+'.iter_mut() {')
   lines.append(f'        *cell{d-1}=Some({v});');lines+=['    }']*d
  else:lines.append(f'    {ref}=Some({v});')
 lines+=['}']
 lines+=[f'fn dump_{k}(out:&mut Dump,p:&str,x:&{typ},tags:&Tags) {{']
 for f in fields:
  n=f['name'];t=f['type'];d=len(f['extents']);key=f'&format!("{{p}}.{n}")';ref='x.'+n
  if n=='param' and k=='pulsar':lines.append(f'    params(out,{key},&{ref},tags);');continue
  if t in names and not f['pointers']:
   if d:lines.append(f'    for (i,cell) in {ref}.iter().enumerate() {{dump_{t}(out,&format!("{{p}}.{n}[{{i}}]"),cell,tags);}}')
   else:lines.append(f'    dump_{t}(out,{key},&{ref},tags);')
  elif f['pointers'] or t.endswith('Func'):
   mapper='tags.matrix(v)' if t=='double' else 'tags.observation(v)' if t=='observation' else 'tags.payload(v)' if f['pointers'] else f'format!("callback:{ {"paramDerivFunc":"deriv","constraintDerivFunc":"constraint","paramUpdateFunc":"update"}[t]}{{}}",v.identity)'
   if d:lines.append(f'    out.pointers({key},{ref}.iter().map(|v|v.as_ref().map(|v|v.as_ref().map_or_else(||"null".into(),|v|{mapper}))));')
   else:lines.append(f'    match &{ref} {{None=>out.unwritten({key}),Some(None)=>out.symbol({key},"null"),Some(Some(v))=>out.symbol({key},&{mapper})}};')
  else:
   cast='map(|v|v.map(|n| n as i32))' if t.startswith('enum ') else ('cloned()' if t=='longdouble' else 'copied()')
   iterator=ref+'.iter()'+'.flatten()'*(d-1)+'.'+cast if d else 'std::iter::once('+ref+('.map(|n|n as i32)' if t.startswith('enum ') else ('.clone()' if t=='longdouble' else ''))+')'
   lines.append(f'    out.values({key},{iterator});')
 lines+=['}']
 if k=='pulsar':
  lines+=['fn natural(out:&mut Dump,p:&str,x:&SourcePulsar,tags:&Tags) {']
  for f in fields:
   n=f['name'];key=f'&format!("{{p}}.{n}")'
   if n=='param':lines.append(f'    params(out,{key},&x.param,tags);')
   elif f['natural_state'].startswith('unwritten'):lines.append(f'    out.unwritten({key});')
   elif f['natural_state'].startswith('source writes string'):lines.append(f'    out.prefix({key},&x.{n}[..]);')
   else:
    # Reuse precisely the generated visitor line for the one source field.
    line=next(l for l in reversed(lines[:lines.index('fn natural(out:&mut Dump,p:&str,x:&SourcePulsar,tags:&Tags) {')]) if key in l)
    lines.append(line)
  lines+=['}']
(ROOT/'rust/src/pulsar_probe_fields.rs').write_text('\n'.join(lines)+'\n')
