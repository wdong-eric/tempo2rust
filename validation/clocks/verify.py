"""Behavior assertions and artifact/disposable-source mutation sensitivity."""
import copy
import json
import re
from profile import HERE, BUILD, save, sha
from run import compile_probe, execute, compare


def ends(case, op):
    return [e for e in case['events'] if e['phase']=='end' and e['operation']==op]


def differs(expected,actual):
    try:compare(expected,actual)
    except AssertionError as error:return str(error)
    raise AssertionError('Mutation escaped the comparator')


def semantic(result):
    # Executable filenames and diagnostic source paths must not kill a mutation.
    return {k:result[k] for k in ('returncode','timeout','events')}


def contracts(rows):
    r={c['id']:c for c in rows}
    checks=[]
    def check(name,condition):
        assert condition,name
        checks.append(name)
    def result(cid,op='clock',index=0):return ends(r[cid],op)[index]['result']
    def value(cid,op='tt'):return result(cid,op)['value'].get('decimal')
    check('F five exact fixtures', [len(result('table_F_'+n,'load')['table']['samples']) for n in ('skipped_row','all_duplicate','duplicate_end','singleton_proxy','decreasing_proxy')]==[2,2,3,1,2])
    check('F duplicate arithmetic remains NaN', all(result('table_F_'+n,'value')['value']=={'class':'NaN'} for n in ('all_duplicate','duplicate_end')))
    check('ordinary inclusive endpoints and adjacent dates',len(ends(r['table_ordinary'],'value'))==9)
    check('binary32 path sum rounding',value('route_binary32_accumulation')=='7' and value('route_binary32_rounding')=='8')
    check('negative acyclic path retained',value('weights_negative_acyclic')=='3')
    check('nonfinite edge classification',all(value('weights_'+n)=='22' for n in ('nan','inf','1e39')) and value('weights_minf')=='11')
    check('overflowing float route returns empty',value('weights_overflow_sum')=='0')
    reverse=ends(r['route_reverse'],'path')[0]['result']
    check('reverse hint never traverses backward',reverse['reverse_hint'] and reverse['indices']==[] and value('route_reverse')=='0')
    check('explicit chain precedence and first cached match',value('explicit_precedes_auto')=='3' and value('explicit_after_auto')=='9' and value('explicit_first_matching')=='3')
    check('full path explicit token still chooses first basename',all(value('explicit_duplicate_'+n)=='10' for n in ('exact','fullpath','arbitrary_path')))
    check('fallback four stages return declared corrections',[value('fallback_'+n+'_w0') for n in ('original','utc_target','original_tt','both','empty')]==['1','2','3','4','0'])
    check('getCorrection has only UTC fallback',value('fallback_original_tt_w0','correction')=='0' and value('fallback_both_w0','correction')=='0')
    first=r['fallback_empty_w0']['stdout'].split('@BEGIN 3 clock\n')[1].split('@END 3 clock')[0]
    check('exact exhausted fallback diagnostic order',re.findall(r'\[(CLK\d+)\]',first)==['CLK3','CLK4','CLK3','CLK5','CLK3','CLK8','CLK3'])
    check('noWarnings=2 retains errors', '[CLK4]' not in r['fallback_empty_w2']['stdout'] and '[CLK3]' in r['fallback_empty_w2']['stdout'])
    check('warning suppression is process-global',r['lifetime_warning_state']['stdout'].count('[DUP1]')==1)
    queries=ends(r['sequential_epochs'],'clock')[0]['table_queries']
    check('actual shifted table epochs', [q['epoch']['decimal'] for q in queries]==['5','6'] and value('sequential_epochs')=='86460')
    check('shift can exceed original validated range',value('sequential_proxy')=='86500' and '[TAB2]' in r['sequential_proxy']['stdout'])
    check('capacity 30 succeeds, 31 resets count but preserves writes',result('capacity_30')['after']['count']==30 and result('capacity_31')['after']['count']==0 and result('capacity_31')['after']['slots'][29]['value']['decimal']=='30')
    check('accumulator independent capacity',value('capacity_31')=='0' and value('capacity_31','correction')=='496')
    check('changed header before lazy load retains indexed edge',value('lifetime_before_load_copy')=='77' and ends(r['lifetime_before_load_copy'],'clock')[0]['clock_state']['functions'][0]['from']=='A')
    check('loaded values survive root, replacement and removal',all([e['result']['value']['decimal'] for e in ends(r[n],'tt')]==['1','1'] for n in ('lifetime_root_extra','lifetime_after_load_copy','lifetime_after_load_replace','lifetime_after_load_remove')))
    check('added files absent from fixed index',value('lifetime_add')=='0')
    for c in rows:
        for e in ends(c,'correction')+ends(c,'tt'):
            a=copy.deepcopy(e['result']['after']);b=e['result']['before']
            for slot in a['slots']:check_mask=slot.pop('changed_bytes');assert set(check_mask)=={'0'}
            compare(a,b,c['id']+'.read_only_observation')
    checks.append('every getCorrection/getCorrectionTT leaves initialized observation state unchanged')
    first,second=ends(r['toa_multiple_pulsars'],'toa')
    check('SAT l-index membership and k-order',[o['after']['count'] for o in first['result']['observations']]==[3,3,3,2,3] and [o['after']['count'] for o in second['result']['observations']]==[3,4,4,2,4])
    check('SAT extra shifted slot within storage',ends(r['toa_capacity_28'],'toa')[0]['result']['observations'][0]['after']['slots'][29]['value']['decimal']=='928')
    check('clockCorr 0 and 2 bypass initialization',not ends(r['toa_bypass_no_initialization'],'toa')[0]['clock_state']['initialized'])
    check('PKS real route has five stored components',result('pks_real')['after']['count']==5 and value('pks_real')=='69.183999495293733')
    return checks


def verify(cases,payload,isolated,src,cc,flags):
    reference={c['id']:c for c in payload['cases']}
    checks=dict(behavior_contracts=contracts(payload['cases']),artifact_mutations=[],source_mutations=[],
                unrun_cases=[c['id'] for c in isolated['cases']],
                scope='Exact recorded-host integrity and source reachability, no Rust tolerance or acceptance')
    # Mutations span real typed artifacts, including values and failure statuses.
    sample=reference['route_simple']
    targets=[('missing_case',payload,lambda x:x['cases'].pop()),
             ('extra_case',payload,lambda x:x['cases'].append(copy.deepcopy(x['cases'][0]))),
             ('case_order',payload,lambda x:x['cases'].reverse()),
             ('missing_event',sample,lambda x:x['events'].pop()),
             ('extra_event',sample,lambda x:x['events'].append(copy.deepcopy(x['events'][0]))),
             ('event_order',sample,lambda x:x['events'].reverse()),
             ('missing_field',sample,lambda x:x['events'][1].pop('table_queries')),
             ('extra_field',sample,lambda x:x['events'][1].update(invented=0)),
             ('component_count',sample,lambda x:ends(x,'clock')[0]['result']['after'].update(count=0)),
             ('destination_order',reference['sequential_epochs'],lambda x:ends(x,'clock')[0]['result']['after']['slots'].reverse()),
             ('binary64_bits',sample,lambda x:ends(x,'tt')[0]['result']['value'].update(bits='0000000000000000')),
             ('nonfinite_class',reference['table_F_all_duplicate'],lambda x:ends(x,'value')[0]['result']['value'].update({'class':'+Infinity'})),
             ('binary32_weight',sample,lambda x:ends(x,'clock')[0]['clock_state']['functions'][0]['badness'].update(binary32='00000000')),
             ('epoch_shift',reference['sequential_epochs'],lambda x:ends(x,'clock')[0]['table_queries'][1].update(epoch=ends(x,'clock')[0]['table_queries'][0]['epoch'])),
             ('cache_loaded_flag',sample,lambda x:ends(x,'clock')[0]['clock_state']['functions'][0].update(loaded=False)),
             ('slot_write_mask',sample,lambda x:ends(x,'clock')[0]['result']['after']['slots'][0].update(changed_bytes='0'*40)),
             ('exit_status',reference['table_bad_header'],lambda x:x.update(returncode=0)),
             ('last_defined_stage',reference['table_bad_header'],lambda x:x['events'][-1].update(phase='end')),
             ('unrun_cannot_be_pass',isolated,lambda x:x['cases'][0].update(status='passed'))]
    for name,expected,mutate in targets:
        changed=copy.deepcopy(expected);mutate(changed)
        checks['artifact_mutations'].append(dict(name=name,rejected=differs(expected,changed)))
    # Each compiled mutant changes defined ordinary behavior only. Canonical source
    # and checked oracle remain untouched. No invalid-access/cycle probe is run.
    mutations=[
        ('table_numeric_prefix','table_permissive',[('tabulatedfunction.C','==2){','==2 && strchr(line,\'j\')==NULL){')]),
        ('table_duplicate_bracket','table_internal_duplicate',[('tabulatedfunction.C','if (samp[imid].x > x)','if (samp[imid].x >= x)')]),
        ('directed_reverse','route_reverse',[('clkcorr.C','e.from = fromNode;','e.from = toNode;'),('clkcorr.C','e.to = toNode;','e.to = fromNode;')]),
        ('route_tie','route_parallel',[('clkcorr.C','if (newDist < dist[edge.to])','if (newDist <= dist[edge.to])')]),
        ('fallback_empty_count','fallback_empty_w0',[('clkcorr.C','          return;\n        }\n      }\n    }\n  }\n\n  double correction','          obs->nclock_correction = 1; return;\n        }\n      }\n    }\n  }\n\n  double correction')]),
        ('sequential_epoch','sequential_epochs',[('clkcorr.C','(double)obs->sat + *totalCorrection / SECDAY','(double)obs->sat + 0.0 / SECDAY')]),
        ('loaded_table_cache','lifetime_after_load_copy',[('clkcorr.C','if (func->loaded)','if (false && func->loaded)')]),
        ('index_lifetime','lifetime_root_extra',[('clkcorr.C','if (g_clockCorrectionsInitialized)','if (false && g_clockCorrectionsInitialized)')]),
        ('sat_jump_index','toa_multiple_pulsars',[('toa2utc.C','psr[p].jumpSAT[l]==1','psr[p].jumpSAT[k]==1')]),
        ('sat_extra_shift','toa_capacity_0',[('toa2utc.C','int icc=obs->nclock_correction;','int icc=obs->nclock_correction-1;')]),
        ('capacity_limit','capacity_30',[('clkcorr.C','obs->nclock_correction >= MAX_CLK_CORR','obs->nclock_correction >= MAX_CLK_CORR-1')]),
        ('table_exit_status','table_bad_header',[('tabulatedfunction.C','fileName);\n        exit(1);\n    }\n    for (c=line;', 'fileName);\n        exit(2);\n    }\n    for (c=line;')])]
    fixtures={c['id']:c for c in cases}
    for name,cid,changes in mutations:
        target,profile=compile_probe(src,cc,flags,'mutation_'+name,changes)
        result=execute(fixtures[cid],target)
        reason=differs(semantic(reference[cid]),semantic(result))
        checks['source_mutations'].append(dict(name=name,case=cid,changes=[dict(source=f,old=a,new=b) for f,a,b in changes],
                                               rejected=reason,build=profile))
        print('source mutation',name,'DETECTED',flush=True)
    checks['status']='PASS'
    return checks
