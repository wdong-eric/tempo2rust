"""U value-initialization, independent of initialiseOne's natural defaults."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
inv=json.loads((ROOT/'rust/verification/pulsar_lifecycle/freeze/field-and-operation-map.json').read_text())
names={'clock_correction':'SourceClockCorrection','observation':'SourceObservation','storePrecision':'StorePrecision','FitOutput':'FitOutput','FitInfo':'FitInfo','pulsar':'SourcePulsar'}
lines=['// W generated caller-only value initialization; never a production default.']
for kind,typ in names.items():
 lines.append(f'fn zero_{kind}(x: &mut {typ}) {{')
 for f in inv['types'][kind]:
  n,t,d=f['name'],f['type'],len(f['extents']);ref='x.'+n
  if t=='parameter':
   lines.append('for s in &mut x.param.slots {s.a_size=Some(0);s.link_from.fill(Some(0));s.link_to.fill(Some(0));s.n_link_to=Some(0);s.n_link_from=Some(0);}')
   continue
  for i in range(d):lines.append(f'for cell{i} in '+(ref if i==0 else 'cell'+str(i-1))+'.iter_mut() {')
  dst=ref if not d else '*cell'+str(d-1)
  if t in names and not f['pointers']:lines.append(f'zero_{t}(&mut {dst});')
  else:
   v='None' if f['pointers'] or t.endswith('Func') else {'double':'0.0','float':'0.0','longdouble':'Float::with_val(128,0)','enum constraint':'Constraint::constraint_dmmodel_mean','enum series_type':'SeriesType::series_undefined'}.get(t,'0')
   lines.append(f'{dst}=Some({v});')
  lines+=['}']*d
 lines.append('}')
 lines.append(f'fn retained_{kind}(a:&{typ},b:&{typ}) {{')
 for f in inv['types'][kind]:
  if kind=='observation' and f['name']=='troposphericDelay':continue
  n=f['name'];lines.append(f'assert_eq!(format!("{{:?}}",a.{n}),format!("{{:?}}",b.{n}),"{kind}.{n}");')
 lines.append('}')
(ROOT/'rust/src/terrestrial_fixture_fields.rs').write_text('\n'.join(lines)+'\n')
