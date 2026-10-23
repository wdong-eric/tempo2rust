"""Declarative controlled inputs, no replacement clock implementation."""
import math
from profile import HERE, save


def table(a='A', b='B', y=1, badness='1', start=0, end=10):
    return f'# {a} {b} {badness}\n{start} {y}\n{end} {y}\n'


def make_cases():
    cases = []
    def add(name, files, actions, classification='ordinary', evidence=''):
        cases.append(dict(id=name, files=files, actions=[['profile'], *actions], classification=classification, evidence=evidence))
    def tbl(name, text, queries, classification='ordinary'):
        add('table_'+name, {'sample.clk':text}, [['load','{case}/sample.clk'], *[['value',str(q)] for q in queries]],classification)
    # F's five inputs are byte-for-byte preserved.
    tbl('F_skipped_row','# A B\n1 0\nbad row\n2 0\n',[1.5])
    tbl('F_all_duplicate','# A B\n1 0\n1 1\n',[1])
    tbl('F_duplicate_end','# A B\n1 0\n2 1\n2 2\n',[2])
    tbl('F_singleton_proxy','# A B\n1 0\n',[0,2])
    tbl('F_decreasing_proxy','# A B\n2 0\n1 0\n',[1.5])
    tbl('ordinary','# A B 1\n50000 -0\n50001 2\n50002 3\n',
        [49999,50000,math.nextafter(50000,math.inf),50000.5,math.nextafter(50001,-math.inf),50001,math.nextafter(50001,math.inf),50002,50003])
    tbl('internal_duplicate','# A B\n1 10\n2 20\n2 30\n3 40\n',[1,math.nextafter(2,-math.inf),2,math.nextafter(2,math.inf),3])
    tbl('decreasing_middle','# A B\n1 10\n3 30\n2 20\n4 40\n',[1,2,2.5,3,4])
    tbl('decreasing_both_proxies','# A B\n3 30\n1 10\n',[0,1,2,3,4])
    tbl('permissive','# A B\n\n# comment\n # leading comment\none\n1junk 4\n1 10junk\n2 0x1p2 trailing\n3 6 # trailing\n',[1,1.5,2,3])
    tbl('signed_zero','# A B\n-0 -0\n1 -0\n',[-1,'-0',0.5,1,2])
    for name, text in [('nan_y','# A B\n1 nan\n2 1\n'),('inf_y','# A B\n1 inf\n2 inf\n'),
                       ('mixed_infinity','# A B\n1 -inf\n2 inf\n'),('nan_x','# A B\nnan 3\n2 4\n'),
                       ('infinite_x','# A B\n-inf 3\ninf 4\n')]:
        tbl(name,text,['-inf',0,1,1.5,2,'inf','nan'])
    tbl('query_nonfinite','# A B\n1 3\n2 5\n',['nan','inf','-inf'])
    tbl('header_255','#'+'h'*253+'\n'+'1 2\n2 4\n',[1.5])
    tbl('body_chunk_boundary','# A B\n'+' '*1023+'1 7\n2 9\n',[1.5])
    add('table_reload',{'first.clk':table(y=3),'second.clk':table('C','D',7)},
        [['load','{case}/first.clk'],['value','5'],['load','{case}/second.clk'],['value','5']])
    for name,text in [('empty',''),('bad_header','A B\n1 2\n2 3\n'),('no_samples','# A B\nbad\n')]:
        tbl(name,text,[], 'exit')
    add('table_missing',{},[['load','{case}/missing.clk']],'exit')
    tbl('singleton_exact','# A B\n1 0\n',[1],'undefined')
    tbl('singleton_nan','# A B\n1 0\n',['nan'],'undefined')
    for name in ('empty_value','empty_start','empty_end'):
        add('table_'+name,{},[[name]],'undefined')
    tbl('header_256','#'+'h'*254+'\n'+'1 2\n2 4\n',[],'undefined')
    tbl('header_800','#'+'h'*798+'\n'+'1 2\n2 4\n',[],'undefined')
    # Host filesystem limits a basename to 255 bytes, which fits fileName[256].
    add('table_basename_255',{'x'*251+'.clk':table()},[['load','{case}/'+'x'*251+'.clk'],['value','5']])
    deep='/'.join(['d'*100]*6)+'/'+'t.clk'
    add('table_long_full_path',{deep:table()},[['load','{case}/'+deep],['value','5']])

    def graph(name, edges, actions, classification='ordinary', extra=None):
        files={'root/clock/'+k:v for k,v in edges.items()}
        files.update({'root/observatory/observatories.dat':'-4554232.1 2816759.1 -3454036.3 Test T A\n',
                      'root/observatory/aliases':'T alias\n'})
        if extra:files.update(extra)
        add(name,files,[['env','{case}/root',''], *actions],classification)
    def obs(sat='5',count=2,i=0,flag=1,tel='T'):
        return ['obs',str(i),str(sat),tel,str(flag),str(count)]
    def call(a='A',b='B',sat='5',warnings=0):
        return [obs(sat),['clock','0',a,b,str(warnings)],['tt','0'],['correction','0',a,b,str(warnings)]]
    ab={'a.clk':table()}
    graph('route_simple',ab,call())
    graph('normalization',{'a.clk':table('a','b')},call(' \ta\r\n',' B '))
    graph('identity_no_initialization',{},call(' a ','A'))
    graph('identity_null_target',{},call(' ','@null'))
    graph('source_null',ab,call('@null'))
    graph('source_empty',ab,call(''))
    graph('source_spaces',ab,call('   '))
    graph('target_null',ab,call('A','@null'))
    graph('target_empty',ab,call('A',''))
    graph('alias',ab,[[*obs()[:3],'alias',*obs()[4:]],['clock','0','','B','0']])
    graph('route_reverse',ab,call('B','A')+[['path','B','A','5']])
    graph('route_missing_nodes',ab,[['path','X','B','5'],['path','A','X','5'],['path','X','Y','5']]+call('X','Y'))
    graph('route_disconnected',{'a.clk':table('A','B'),'c.clk':table('C','D')},call('A','D')+[['path','A','D','5']])
    for label,es in [('parallel',{'a.clk':table(y=11),'z.clk':table(y=22)}),
                     ('node_tie',{'a.clk':table('A','C',1),'b.clk':table('A','B',2),'c.clk':table('B','D',4),'d.clk':table('C','D',8)})]:
        graph('route_'+label,es,call('A','B' if label=='parallel' else 'D'))
    graph('route_binary32_rounding',{'a.clk':table('A','B',1,'16777216'),'b.clk':table('B','D',2,'1'),'z.clk':table('A','D',8,'16777217')},call('A','D'))
    graph('route_binary32_accumulation',{'a.clk':table('A','B',1,'16777216'),'b.clk':table('B','C',2,'1'),'c.clk':table('C','D',4,'1'),'z.clk':table('A','D',8,'16777218')},call('A','D'))
    graph('route_block_retry',{'a.clk':table(y=1,start=0,end=4),'b.clk':table(y=2,badness='2',start=4,end=10)},call(sat='5')+call(sat='3')+call(sat='4')+call(sat='10')+call(sat='11'))
    graph('route_invalid_two_edges',{'a.clk':table('A','C',1,start=0,end=4),'b.clk':table('C','B',1,start=6,end=10),'z.clk':table(y=9,badness='5')},call())
    graph('range_disjoint',{'a.clk':table('A','C',1,start=0,end=4),'b.clk':table('C','B',1,start=6,end=10)},[['define','a.clk b.clk','0']]+call())
    for name,start,end,epoch in [('negative',-10,-1,-5),('above_million',1000001,1000010,1000005)]:
        graph('range_'+name,{'a.clk':table(start=start,end=end)},call(sat=epoch)+call(sat=epoch))
    graph('range_nan_query',ab,call(sat='nan'))
    for weight in ['-1','0','nan','inf','-inf','3.4028234663852886e38','1e39','garbage','']:
        label=weight.replace('.','_').replace('+','p').replace('-','m') or 'absent'
        graph('weights_'+label,{'a.clk':table(y=11,badness=weight),'z.clk':table(y=22,badness='2')},call())
    graph('weights_overflow_sum',{'a.clk':table('A','C',1,'3e38'),'b.clk':table('C','B',2,'3e38')},call())
    graph('weights_negative_acyclic',{'a.clk':table('A','C',1,'2'),'b.clk':table('C','B',2,'-3'),'z.clk':table('A','B',8,'4')},call())
    graph('weights_cycle',{'a.clk':table('A','B',1,'1'),'b.clk':table('B','C',2,'-2'),'c.clk':table('C','B',4,'-2'),'d.clk':table('C','D',8,'5')},call('A','D'),'bounded')
    graph('path_identity',ab,[['path','A','A','5']])
    graph('index_missing_direct',{},[['index_file','{case}/missing.clk','0']],'exit')
    graph('index_extra_without_default',{},[['env','{case}/root','{case}/extra'],['index','0']],'exit',{'extra/a.clk':table()})
    graph('index_extra_offset',{'same.clk':table(y=20)},[['env','{case}/root','{case}/extra']]+call(),extra={'extra/same.clk':table(y=10)})
    graph('index_empty_extra',ab,[['env','{case}/root','{case}/missing_extra']]+call())
    graph('index_sorted',{'z.clk':table(y=20),'a.clk':table(y=10)},call())
    for name,text in [('empty',''),('invalid','A B\n'),('short','# A\n'),('leading_space',' # A B\n')]:
        graph('index_'+name,{'a.clk':text},call(), 'exit')
    graph('index_absent',{},call(),'exit')
    graph('index_env_unset',ab,[['env','@unset','']]+call(),'exit')
    graph('index_env_empty',ab,[['env','','']]+call(),'exit')
    graph('index_bad_lazy_table',{'a.clk':'# A B\n'},[['index','0']]+call(),'exit')
    graph('index_missing_file',ab,[['remove','{case}/root/clock/a.clk'],['index','0']],'exit')
    graph('headers_long_names',{'a.clk':table('A','B'*63)},call('A','B'*63))
    graph('headers_badness_prefix',{'a.clk':table(badness='2junk')},call())
    for name,tokens in [('empty',''),('null','@null'),('missing','missing.clk'),('broken','b.clk a.clk')]:
        graph('explicit_'+name,{'a.clk':table('A','C'),'b.clk':table('C','B')},[['define',tokens,'0']],'exit')
    edges={'a.clk':table('A','C',1),'b.clk':table('C','B',2),'z.clk':table('A','B',9,'0.5')}
    graph('explicit_precedes_auto',edges,[['define',' a.clk\t b.clk ','0']]+call())
    graph('explicit_after_auto',edges,call()+[['define','a.clk b.clk','0']]+call())
    graph('explicit_first_matching',edges,[['define','a.clk b.clk','0'],['define','z.clk','0']]+call())
    for name,token in [('exact','same.clk'),('fullpath','{case}/root/clock/same.clk'),('arbitrary_path','/nonexistent/same.clk')]:
        graph('explicit_duplicate_'+name,{'same.clk':table(y=20)},[['env','{case}/root','{case}/extra'],['define',token,'0']]+call(),extra={'extra/same.clk':table(y=10)})
    graph('helper_broken',ab,[['define','a.clk','0'],obs(),['helper_fill','0','0','X','B','0'],['helper_accumulate','0','0','X','B','0']])
    graph('helper_early_target',edges,[['define','a.clk b.clk','0'],obs(count=0),['helper_fill','0','0','A','C','0'],['helper_accumulate','0','0','A','C','0']])
    for name,es in [('original',{'a.clk':table('A','Z',1)}),('utc_target',{'a.clk':table('UTC','Z',2)}),
                    ('original_tt',{'a.clk':table('A','TT(TAI)',3)}),('both',{'a.clk':table('UTC','TT(TAI)',4)}),
                    ('empty',{'a.clk':table('P','Q',5)})]:
        for warn in (0,1,2,-1):
            graph('fallback_'+name+'_w'+str(warn),es,call('A','Z',warnings=warn)+call('A','Z',warnings=warn))
    graph('fallback_utc_identity',ab,call('X','UTC'))
    graph('fallback_tt_identity',ab,call('TT(TAI)','Z'))
    graph('sequential_epochs',{'a.clk':table('A','C',86400),'b.clk':'# C B\n0 0\n10 100\n'},call(sat='5.000000000000000000000000000000001'))
    graph('sequential_proxy',{'a.clk':table('A','C',86400),'b.clk':'# C B\n0 0\n5 100\n'},call(sat=5))
    graph('sequential_nan',{'a.clk':table('A','C','nan'),'b.clk':table('C','B',2)},call())
    for label,y in [('positive','inf'),('negative','-inf')]:
        graph('sequential_'+label+'_infinity',{'a.clk':f'# A C\n0 0\n10 {y}\n','b.clk':table('C','B',2)},call())
    graph('sequential_inf',{'a.clk':table('A','C','inf'),'b.clk':table('C','B',2)},call())
    for n in (29,30,31):
        es={f'{i:02}.clk':table(f'C{i}',f'C{i+1}',i+1,start=0,end=100) for i in range(n)}
        graph('capacity_'+str(n),es,call('C0',f'C{n}'))
    graph('independent_sum',ab,[obs(count=4),['slot','0','0','1e16','A'],['slot','0','1','1','B'],['slot','0','2','-1e16','C'],['slot','0','3','-0','D'],['tt','0'],['correction','0','A','B','0'],['tt','0']])

    # Resource edits are actions inside the same process. Initial files restored before each fresh replay.
    mutations={'replacement.clk':table('X','Y',77),'same_header.clk':table(y=88),
               'root2/clock/a.clk':table(y=99),'extra/b.clk':table(y=55),
               'new.clk':table('A','Z',66)}
    graph('lifetime_root_extra',ab,call()+[['env','{case}/root2','{case}/extra']]+call(),extra=mutations)
    for when in ('before_load','after_load'):
        for operation in ('copy','replace','remove'):
            actions=[['index','0']] if when=='before_load' else call()
            if operation=='remove':actions+=[['remove','{case}/root/clock/a.clk']]
            else:actions+=[[operation,'{case}/replacement.clk','{case}/root/clock/a.clk']]
            graph('lifetime_'+when+'_'+operation,ab,actions+call(), 'exit' if when=='before_load' and operation=='remove' else 'ordinary',mutations)
    graph('lifetime_add',ab,[['index','0'],['copy','{case}/new.clk','{case}/root/clock/new.clk']]+call('A','Z'),extra=mutations)
    graph('lifetime_fresh_replacement',{'a.clk':table('X','Y',77)},call('X','Y'))
    graph('lifetime_changed_samples_before_load',ab,[['index','0'],['copy','{case}/same_header.clk','{case}/root/clock/a.clk']]+call(),extra=mutations)
    graph('lifetime_warning_state',ab,call('X','UTC')+call('Y','UTC')+call('X','UTC'))

    # Controlled SAT jump flags deliberately differ at l and k.
    toa=[['pulsar','0','3','3','A','B','0'],['pulsar','1','2','2','A','B','2']]
    toa += [obs(i=i,flag=f) for i,f in enumerate((1,0,2,1,0))]
    for p in (0,1):
        toa += [['jump',str(p),str(k),str(k/4),str(flag),'J'+str(k)] for k,flag in [(0,1),(1,0),(2,1),(3,1)]]
    toa += [['members','0','3','1','2'],['members','1','2','1'],['members','2','1'],['members','3','2','1'],['members','4','1','1'],['toa','2'],['toa','2']]
    graph('toa_multiple_pulsars',ab,toa)
    graph('toa_bypass_no_initialization',{},[['pulsar','0','2','0','A','B','0'],obs(i=0,flag=0),obs(i=1,flag=2),['toa','1']])
    graph('toa_nonstandard_flags',ab,[['pulsar','0','2','0','A','B','0'],obs(i=0,flag=3),obs(i=1,flag=-1),['toa','1']])
    graph('toa_duplicate_memberships',ab,[['pulsar','0','1','1','A','B','0'],obs(flag=0,count=0),['jump','0','0','0','1','unused'],['jump','0','1','0.25','1','repeat'],['members','0','1','1'],['toa','1']])
    graph('toa_site_source',ab,[['pulsar','0','1','0','','B','0'],obs(),['toa','1']])
    for count in (0,28,29):
        graph('toa_capacity_'+str(count),ab,[['pulsar','0','1','1','A','B','0'],obs(count=count,flag=0),['jump','0','0','0','1','unused'],['jump','0','1','0.25','0','j'],['members','0','1'],['toa','1']], 'undefined' if count==29 else 'ordinary')
    graph('toa_long_jump_label',ab,[['pulsar','0','1','1','A','B','0'],obs(flag=2),['jump','0','0','0','1','unused'],['jump','0','1','-0','0','L'*80],['members','0','1'],['toa','1']])
    add('pks_real',{},[['env','{real}',''],obs('58000.12345678901234567890123456789',tel='PKS'),['clock','0','','TT(TAI)','0'],['tt','0'],['correction','0','UTC(PKS)','UTC','0'],['pulsar','0','1','0','','TT(TAI)','0'],['toa','1'],['toa','1']])
    graph('cvs_and_debug',ab,[['debug','1','1']]+call()+[['pulsar','0','1','0','A','B','0'],['toa','1']])
    for c in cases:
        c['last_action'] = len(c['actions'])-1
        if c['classification']=='exit':
            operations=[a[0] for a in c['actions']]
            terminal=next(op for op in ('load','clock','define','index','index_file') if op in operations)
            c['last_action']=operations.index(terminal)
    assert len({c['id'] for c in cases}) == len(cases)
    return cases


if __name__=='__main__':
    cases=make_cases();save(HERE/'cases.json',cases)
    print(f'{len(cases)} cases, {sum(len(c["actions"]) for c in cases)} ordered actions')
