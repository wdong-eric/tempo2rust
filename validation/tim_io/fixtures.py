"""Declared actions and classifications, generated before observations are recorded."""
import struct

def row(name='row',sat='60000.125',err='3',freq='1400',site='PKS',flags=''):
    return f'{name} {freq} {sat} {err} {site} {flags}\n'

def parkes(name='park',sat='60000.125',err='3',freq='1400',phase='0.25',site='7'):
    s=list(' '*80)
    for a,b,v in [(1,25,name),(25,34,freq),(34,55,sat),(55,63,phase),(63,71,err),(79,80,site)]:s[a:b]=list(v.ljust(b-a))
    return ''.join(s)+'\n'

def princeton(dm='1.25',sat='60000.125',err='3',freq='1400'):
    s=list(' '*79)
    for a,b,v in [(0,1,'7'),(15,24,freq),(25,45,sat),(45,54,err),(68,78,dm)]:s[a:b]=list(v.ljust(b-a))
    return ''.join(s)+'\n'

def itoa(sat='60000.125',phase='0.25',freq='1400',err='3'):
    s=list('X'*9+' '*51)
    for a,b,v in [(9,28,sat),(28,34,err),(34,45,freq),(45,55,phase),(57,60,'PKS')]:s[a:b]=list(v.ljust(b-a))
    return ''.join(s)+'\n'

def record(t,data):
    assert len(data)==200
    return struct.pack('>ii',204,t)+data+struct.pack('>i',204)

def jbo():
    header=b''.join(s.ljust(8,b' ') for s in [b'HEADER',b'creator',b'ver1',b'user'])+b' '*168
    source=b'SOURCE  NAME    PART3   '+b' '*176
    site=b'JBO site name ends here'.ljust(24,b' ')+b''.join(struct.pack('>d',v) for v in [1.,2.,3.])+b' '*152
    assert len(site)==200
    vals=[1230101,43200.123456789,2.5,1400,1]+[0]*20
    vals[14:18]=[11.,12.,13.,14.];vals[21]=15;vals[23]=1230102;vals[24]=43201
    data=b''.join(b'TEXTDATA' if i==13 else struct.pack('>d',v) for i,v in enumerate(vals))
    return record(3,header)+record(0,source)+record(1,site)+record(2,data)+record(9,b'\0'*200)

def cases():
    cs=[]
    def add(id,actions,files=None,cls='defined',why='',rc=0,outputs=None):
        cs.append(dict(id=id,classification=cls,rationale=why,actions=[[str(x) for x in a] for a in actions],files={k:(v.encode() if isinstance(v,str) else v).hex() for k,v in (files or {}).items()},outputs=outputs or [],returncode=rc,state='SOURCE_MAP.md caller seed; fresh process, two pulsars, twelve initialized observation slots; ordered actions are full input'))
    def tim(id,text,cls='defined',actions=None,more=None,rc=0,why=''):
        add(id,actions or [['read',0,'{case}/input.tim']],{'input.tim':text,**(more or {})},cls,why,rc)
    tim('modern_basic','FORMAT 1\n'+row())
    tim('modern_precisions','FORMAT 1\n'+row('precise','60000.12345678901234567890123456789',flags='-addsat 0.1234567890123456789012345')+row('exponent','60000e-2')+row('split','6.0000e+4'))
    for site in ['@','@suffix','bat','BaT','STL','STL_FBAT','stl','PKS']:
        tim('site_'+site.replace('@','at'),'FORMAT 1\n'+row(site=site))
    tim('flags','FORMAT 1\n'+row(flags='-a one -a two -n -2 -dm 10 -dme 0.5 -addsat 2 -addsat -1 -to 3 -selectAll yes'))
    tim('flag_spaces','FORMAT 1\n'+row(flags='-a    alpha -b\tbeta -end'))
    tim('flag_hyphen_scan','FORMAT 1\n'+row(name='ab-c',flags='-a abc-def -n -1e-2'))
    tim('max_flags','FORMAT 1\n'+row(flags=' '.join('-a x' for _ in range(40))),cls='normal_exit',rc=1,why='40th in-bounds write followed by capacity exit')
    tim('info_max_flags','FORMAT 1\nINFO 3\n'+row(flags=' '.join('-a x' for _ in range(39))),cls='normal_exit',rc=1)
    tim('capacity','FORMAT 1\n'+row()+row('two')+row('three'),actions=[['set',0,'maxobs',4],['read',0,'{case}/input.tim']],cls='normal_exit',rc=1)
    tim('comments_deleted','FORMAT 1\nC ignored\n# ignored\nI '+row('deleted')+row('normal')+'c lowercase\n C indented\n')
    tim('no_final_newline','FORMAT 1\n'+row().rstrip('\n'))
    tim('whitespace','FORMAT 1\n\n \t \n\t'+row()[:-1]+'\r\n')
    tim('partial_modern','FORMAT 1\npartial 123\n'+row()+'tail 456\n')
    tim('chunk_comment','FORMAT 1\n#'+'x'*997+'\n'+row())
    tim('chunk_split','FORMAT 1\n#'+'x'*998+row())
    tim('profile','FORMAT 1\nPROFILE_DIR profiles\n'+row()+ 'I '+row('deleted'))
    tim('profile_truncate','FORMAT 1\nPROFILE_DIR '+'p'*499+'\n'+row())
    for name,text in [('parkes',parkes()),('princeton',princeton()),('princeton_no_dm',princeton(dm='none')),('itoa',itoa()),('parkes_partial',parkes(freq='bad')),('princeton_partial',princeton(err='bad')),('itoa_partial',itoa(freq='bad'))]:
        tim(name,text,why='All legacy column source pointers within initialized fgets data')
    tim('legacy_comments','C comment\nc comment\n'+parkes())
    tim('legacy_directives','EFAC 2\n'+parkes()+'JUMP\n'+princeton()+itoa())
    tim('fixed_skip','ignored\nFORMAT 1\n'+parkes(),actions=[['set',0,'fixedFormat',2],['read',0,'{case}/input.tim']])
    tim('head','HEAD\nignored\nTOAS\n'+parkes())
    tim('head_case_end','HEAD\ntoas\n'+princeton())
    tim('selector_format_last','HEAD\nTOAS\nFORMAT 1\n'+row())
    tim('selector_head_last','FORMAT 1\nHEAD\nTOAS\n'+parkes())
    tim('selector_in_comment','# FORMAT token anywhere\n'+row())
    tim('selector_lowercase','format 1\n'+parkes())
    tim('selector_head_token','note HEAD\nTOAS\n'+itoa())
    directives={'GLOBAL_EFAC':'2.5','EFAC':'2','EFLOOR':'5','T2EFAC':'-be test 1.25','T2EQUAD':'-be test 2.75','EMAX':'2','EMIN':'4','ESET':'7','FMAX':'1000','FMIN':'1500','INFO':'4','EQUAD':'4','SIGMA':'6','TIME':'2','MODE':'1','JUMP':''}
    for key,value in directives.items():tim('directive_'+key,'FORMAT 1\n'+key+' '+value+'\n'+row())
    tim('phase','FORMAT 1\n'+row()+'PHASE -2\n'+row('after'))
    tim('order','FORMAT 1\nEQUAD 4\nSIGMA 6\nEFAC 2\nESET 7\nTIME 1\nTIME 2\nEFLOOR 8\nEMAX 7\nINFO 9\n'+row())
    tim('order_without_eset','FORMAT 1\nEQUAD 4\nEFAC 2\n'+row())
    tim('zero_controls','FORMAT 1\nEQUAD 0\nEFAC 0\nSIGMA 0\nTIME 0\nESET 0\n'+row(err='-3'))
    tim('error_thresholds','FORMAT 1\nEMAX 3\nEMIN 3\n'+row()+row('below',err='2')+row('above',err='4'))
    tim('frequency_thresholds','FORMAT 1\nFMIN 1400\nFMAX 1400\n'+row()+row('low',freq='1399')+row('high',freq='1401'))
    tim('skip','FORMAT 1\nSKIP\nINFO 88\nEFAC 12\nJUMP\n'+row('skipped')+'NOSKIP\n'+row('kept')+'END\n'+row('ended'))
    tim('malformed_initialized','FORMAT 1\nEFAC 2\nEFAC bad\nEQUAD nope\nGLOBAL_EFAC nope\nINFO bad\nMODE bad\nPROFILE_DIR\nINCLUDE\n'+row(),why='Only already initialized numeric/array scan destinations; malformed TIME withheld')
    tim('noise_repeat','FORMAT 1\nGLOBAL_EFAC 3\nT2EFAC -be x 2\nT2EQUAD -be x 4\nMODE 1\n'+row(),actions=[['full','{case}/input.tim','{case}/input.tim'],['full','{case}/input.tim','{case}/input.tim']])
    tim('direct_repeat','FORMAT 1\nJUMP\nINFO 7\n'+row(),actions=[['read',0,'{case}/input.tim'],['read',0,'{case}/later.tim'],['read',1,'{case}/later.tim']],more={'later.tim':'FORMAT 1\n'+row('later')})
    tim('full_jump_reset','FORMAT 1\nJUMP\n'+row(),actions=[['full','{case}/input.tim','{case}/input.tim'],['full','{case}/input.tim','{case}/input.tim']])
    tim('include_history','FORMAT 1\nEFAC 2\nTIME 1\nINFO 3\nJUMP\n'+row('parent1')+'INCLUDE child/sub.tim\n'+row('parent2'),more={'child/sub.tim':'FORMAT 1\nEFAC 3\nTIME 2\nINFO 4\nJUMP\n'+row('child1')+'INCLUDE deep.tim\n'+row('child2'),'child/deep.tim':'FORMAT 1\nINFO 5\nJUMP\n'+row('deep')})
    tim('missing_child','FORMAT 1\nINCLUDE missing.tim\n',cls='normal_exit',rc=1)
    add('missing_tim',[['read',0,'{case}/missing.tim']],cls='normal_exit',rc=1)
    # Exact binary64 0.001, tested around sat=60000 with finite binary128 rounding.
    threshold='0.001000000000000000020816681711721685132943093776702880859375'
    tim('deletion_boundary','FORMAT 1\n'+row('equal',sat='60000.'+threshold.split('.')[1])+row('below',sat='60000.001')+row('above',sat='60000.001000000000000001'),actions=[['set',0,'deleteFileName','{case}/delete.txt'],['full','{case}/input.tim','{case}/input.tim']],more={'delete.txt':'60000\n'})
    tim('missing_delete','FORMAT 1\n'+row(),actions=[['set',0,'deleteFileName','{case}/missing'],['full','{case}/input.tim']])
    for n in [0,1,4]:
        add('sort_'+str(n),[['set',0,'nobs',n],['set',0,'tcheck',1],['sort',0],['sort',0]])
    add('sort_reverse',[['set',0,'nobs',4],['obs',0,0,'sat','60003'],['obs',0,1,'sat','60002'],['obs',0,2,'sat','60001'],['obs',0,3,'sat','60000'],['obs',0,1,'deleted',1],['sort',0]])
    add('sort_ties',[['set',0,'nobs',4],['obs',0,0,'sat','51002'],['obs',0,3,'sat','51002'],['sort',0]],cls='host',why='Exact host qsort permutation only; ties are not a portable stable-order guarantee')
    tim('sort_append','FORMAT 1\n'+row(sat='50000'),actions=[['set',0,'nobs',2],['sort',0],['read',0,'{case}/input.tim'],['sort',0],['set',0,'sorted',0],['sort',0]])
    for x,y in [('1','2'),('2','1'),('1','1'),('nan','1'),('1','nan'),('-0','0'),('-inf','inf')]:add('compare_'+x+'_'+y,[['compare',x,y]])
    for s in ['abc\n','abc\r\n','abc','\n']:add('remove_'+s.encode().hex(),[['remove',s]])
    base=[['set',0,'nobs',1]]
    for fmt in ['tempo2','TEMPO2','legacy']:
        add('writer_'+fmt,base+[['obs',0,0,'fname','long_filename_more_than_twenty_two_characters'],['write',0,'{case}/out.tim',fmt]],cls='host',why='ld_fprintf variadic cleanup defect',outputs=['out.tim'])
    add('writer_empty',[['write',0,'{case}/out.tim','tempo2']],cls='host',why='Initialized obsn[0] header backing',outputs=['out.tim'])
    add('writer_fail',base+[['write',0,'{case}/absent/out.tim','tempo2']])
    for name,efac,equad,err in [('nan',1,5,3),('zero',1,3,3),('zero_efac',0,0,3),('negative_efac',-2,1,3)]:
        add('writer_error_'+name,base+[['obs',0,0,'efac',efac],['obs',0,0,'equad',equad],['obs',0,0,'origErr',err],['write',0,'{case}/out.tim','tempo2']],cls='host',outputs=['out.tim'])
    tim('writer_headers','FORMAT 1\nMODE 1\nGLOBAL_EFAC 3\nT2EFAC -be x 2\nT2EQUAD -be x 4\nEQUAD 4\nEFAC 2\n'+row()+'EQUAD 0\nEFAC 1\nI '+row('deleted'),cls='host',actions=[['read',0,'{case}/input.tim'],['write',0,'{case}/out.tim','tempo2']],more={},why='Writer raw bytes and mutation',)
    cs[-1]['outputs']=['out.tim']
    offsets=[['offset',0,2,0,0,0,0,'0',''],['offset',0,3,0,0,0,0,'Parkes','-no absent -be x'],['offset',0,100,1400,0,0,0,'0',''],['offset',0,100,0,1400,0,0,'0',''],['offset',0,100,0,0,60000,0,'0',''],['offset',0,100,0,0,0,60000,'0',''],['offset',0,100,0,0,0,0,'JBO',''],['offset',0,100,0,0,0,0,'0','-be absent']]
    tim('writer_inversions','FORMAT 1\n'+row(sat='60000.0',flags='-be x -to 4 -to 1 -addsat 0.1234567890123456789 -selectALL yes'),cls='host',actions=[['read',0,'{case}/input.tim'],['obs',0,0,'sat',60000],['obs',0,0,'phaseOffset',0.25],*offsets,['write',0,'{case}/out.tim','tempo2'],['write',0,'{case}/again.tim','tempo2']],why='Explicit writer caller state, not evidence of preprocessing')
    cs[-1]['outputs']=['out.tim','again.tim']
    for fmt in ['tempo2','legacy']:
        tim('roundtrip_'+fmt,'FORMAT 1\n'+row(flags='-addsat 1.123456789012345678901 -selectAll 1'),cls='host',actions=[['read',0,'{case}/input.tim'],['write',0,'{case}/out.tim',fmt],['read',1,'{case}/out.tim']]);cs[-1]['outputs']=['out.tim']
    add('writer_nonpositive_f0',base+[['set',0,'F0',0],['write',0,'{case}/out.tim','tempo2']],cls='host',outputs=['out.tim'])
    for date in [101,40228,40229,40301,1000228,1000229,1000301,1230101,1231231,2000301,1240100,1241301]:add('date_'+str(date),[['date',date]],why='Bounded source arithmetic including day 0/month 13 (no out-of-array access)')
    for k in ['4','8','double']:add('swap_'+k,[['swap',k,'ABCDEFGH']],cls='defined' if k=='4' else 'host')
    add('makechars',[['chars',''.join(chr(33+i%90) for i in range(200))]])
    add('jbo_records',[['set',0,'nobs',3],['jbo',0,'{case}/records.bat'],['jbo',0,'{case}/records.bat']],{'records.bat':jbo()},cls='host',why='Pinned raw-record type-punning/alignment; complete ignored final record prevents unsafe reused data')
    add('jbo_multi',[['set',0,'jboFormat',1],['full','{case}/records.bat','{case}/records.bat']],{'records.bat':jbo()},cls='host')
    add('jbo_zero_final',[['jbo',0,'{case}/records.bat']],{'records.bat':record(0,b'ABCDEF  '*25)},cls='host',why='Final feof iteration repeats type 0 log from initialized prior record')
    for mode,endian in [(0,'<'),(1,'>')]:
        data=b'ABCD'+struct.pack(endian+'ifd',-12345,1.25,-2.5)+struct.pack('<i',0x01020304)
        actions=[['open','{case}/primitive.bin suffix'],['swapmode',mode],['primitive','char',4],['primitive','int'],['primitive','float'],['primitive','double'],['primitive','record'],['primitive','char',2],['close'],['open','{case}/primitive.bin'],['close']]
        add('io_sequence_'+str(mode),actions,{'primitive.bin':data},cls='host',why='Union representations and EOF->char conversion; selected swap input is not alternate host')
        data=b''.join(struct.pack(endian+'f',v) for v in [-0.,float('inf'),float('-inf'),float('nan')])+b''.join(struct.pack(endian+'d',v) for v in [-0.,float('inf'),float('-inf'),float('nan')])
        add('io_classes_'+str(mode),[['open','{case}/special.bin'],['swapmode',mode]]+[['primitive','float']]*4+[['primitive','double']]*4+[['close']],{'special.bin':data},cls='host')
    add('io_missing',[['open','{case}/missing.bin']],cls='normal_exit',rc=1)
    tim('diagnostics','FORMAT 1\nMODE 1\nMODE 1\nMODE 1\nTIME 2\n'+row(),actions=[['set',0,'debug',1],['set',0,'cvs',1],['full','{case}/input.tim']])
    tim('legacy_no_newline',parkes().rstrip('\n'))
    tim('legacy_include','INCLUDE child.tim\n'+parkes(),more={'child.tim':princeton()})
    tim('legacy_short',' short\n',why='Nonempty short Parkes candidate is rejected before fixed-column reads')
    tim('parkes_bad_phase',parkes(phase='bad'))
    tim('parkes_bad_error',parkes(err='bad'))
    tim('princeton_bad_freq',princeton(freq='bad'))
    tim('itoa_bad_error',itoa(err='bad'))
    tim('itoa_bad_phase',itoa(phase='bad'))
    tim('legacy_unknown','UNKNOWN_RECORD_LONG_ENOUGH\n')
    tim('head_without_toas','HEAD\nnot_toas\n',why='Nonempty initialized line retained on EOF; no observations')
    tim('flag_missing_value','FORMAT 1\n'+row(flags='-end'))
    tim('flag_punctuation','FORMAT 1\n'+row(flags='-. value'))
    tim('floor_below_error','FORMAT 1\nEFLOOR 2\n'+row())
    tim('noskip_unskipped','FORMAT 1\nNOSKIP\n'+row())
    tim('delete_debug','FORMAT 1\n'+row(),actions=[['set',0,'debug',1],['set',0,'deleteFileName','{case}/delete.txt'],['full','{case}/input.tim']],more={'delete.txt':'60000.125\n'})
    add('sort_quiet',[['set',0,'quiet',1],['set',0,'nobs',3],['sort',0]])
    add('jbo_quiet',[['set',0,'quiet',1],['set',0,'jboFormat',2],['full','{case}/records.bat']],{'records.bat':jbo()},cls='host')
    add('jbo_cvs',[['set',0,'cvs',1],['jbo',0,'{case}/records.bat']],{'records.bat':jbo()},cls='host')
    add('writer_bounded_offset',base+[['offset',0,2,600,800,50000,52000,'pks_alias','ignored -1 -. absent -retained caller'],['write',0,'{case}/out.tim','tempo2']],cls='host',outputs=['out.tim'])
    add('writer_noflags',base+[['obs',0,0,'nFlags',0],['write',0,'{case}/out.tim','tempo2']],cls='host',outputs=['out.tim'])
    tim('modern_nan','FORMAT 1\n'+row(sat='60000nan',freq='nan',err='nan'),why='strtoflt128 consumes numeric prefix independently; native NaN error/frequency are defined arithmetic')
    add('writer_overwrite',base+[['write',0,'{case}/out.tim','tempo2']],{'out.tim':'OLD_FILE_WITH_TRAILING_BYTES'*30},cls='host',outputs=['out.tim'])
    tim('roundtrip_to_flag','FORMAT 1\n'+row(flags='-to 2 -addsat -3'),cls='host',actions=[['read',0,'{case}/input.tim'],['write',0,'{case}/out.tim','tempo2'],['read',1,'{case}/out.tim']]);cs[-1]['outputs']=['out.tim']
    tim('roundtrip_legacy_phase',parkes(phase='0.25'),cls='host',actions=[['read',0,'{case}/input.tim'],['write',0,'{case}/out.tim','tempo2'],['read',1,'{case}/out.tim']]);cs[-1]['outputs']=['out.tim']
    add('io_eof_values',[['open','{case}/eof.bin'],['primitive','char',0],['primitive','char',1],['primitive','int'],['primitive','float'],['primitive','double'],['close']],{'eof.bin':b'X'},cls='host',why='Complete byte assignments via fgetc EOF conversion; read_record_int2 partial fread remains unrun')
    tim('deleted_missing_site','FORMAT 1\nI deleted 1400 60000.5 3\n',why='Five successful I scans accept caller-initialized retained telID; no natural default is inferred')
    tim('mode_suppressed','FORMAT 1\nMODE 1\n'+row(),actions=[['set',0,'noWarnings',2],['read',0,'{case}/input.tim']])
    tim('mode_unsuppressed_plain','FORMAT 1\nMODE 1\nMODE 1\n'+row(),actions=[['set',0,'noWarnings',-1],['read',0,'{case}/input.tim']])
    for c in cs:
        if c['id'] in ['directive_MODE','malformed_initialized','noise_repeat','diagnostics']:
            c['classification']='host'
            c['rationale']+='; TKlog colored warning forms buffer-1 before restoring pointer; bounded host diagnostic'
    return cs
