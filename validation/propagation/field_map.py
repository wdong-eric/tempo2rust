"""Cross-reference source member uses with the complete accepted typed declarations."""
import json
import gzip
import re
from inventory import HERE,save,uncomment,sha

def main():
 frozen=HERE/'outputs/frozen/source_map.json';source=json.loads(frozen.read_text());rows=[]
 trace=HERE/'outputs/record_raw/counts_-1.jsonl.gz'
 events=[json.loads(s) for s in gzip.decompress(trace.read_bytes()).splitlines()]
 measured=next(e['result'] for e in events if e.get('op')=='profile' and e.get('phase')=='end')
 for routine in source['routines']:
  text=uncomment('\n'.join(routine['source']))
  names=set(re.findall(r'(?:\.|->)([A-Za-z_]\w*)',text))
  declarations=[dict(owner=kind,**field) for kind,fields in source['types'].items() for field in fields if field['name'] in names]
  rows.append(dict(file=routine['file'],routine=routine['signature'],start=routine['start'],end=routine['end'],
   member_declarations=declarations,parameter_indices=sorted(set(re.findall(r'\bparam_[A-Za-z0-9_]+',text))),
   note='source member occurrences and matching header declarations; whole-record copies include all nested fields'))
 save(HERE/'outputs/field_map.json',dict(source_map_sha256=sha(frozen.read_bytes()),routines=rows,complete_types=source['types'],
  globals={'veryFast':'source input; only exact 1 breaks before loop increment',
   'MAX_OBSN':'caller maximum equals six allocated cells per owner',
   'TK_errorCount/TK_warnCount/TK_errorlog/TK_warnlog':'real process-static counters and complete initialized buffers; observer audits saturation retention',
   'dm_delays::t':'function-static 0xFFFF warning mask observed by registered address'},
  caller_backing={'pulsars':2,'observations_per_owner':6,'selected_parameter_slots_per_owner':26,'cells_per_selected_parameter':16},
  precision={'double':measured[:2],'builtin_long_double':measured[2:4],'configured_longdouble':measured[4:6]},native_type_trace_sha256=sha(trace.read_bytes())))
if __name__=='__main__':main()
