"""Reject false locations and retain ordered/numeric diagnostic content (0107)."""
from pathlib import Path
import unittest
from diagnostic_attribution import Attribution
from attribute_generated_output import attribute

ROOT=Path(__file__).resolve().parents[2]


class DiagnosticAttribution(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.mapping=Attribution(ROOT)

    def test_location_then_body_and_order(self):
        m=self.mapping
        (file,line),native=next((site,value) for site,value in m.sites.items() if site[0]=='general_fitting.rs')
        prefix=f'[{file}:{line}]'.encode();old=f'[{native[0]}:{native[1][0]}]'.encode()
        before=old+b' value=1.25 user-file.C\n'+old+b' next\n'
        after=prefix+b' value=1.25 user-file.C\n'+prefix+b' next\n'
        expected=m.stream(before,'native')
        self.assertEqual(m.stream(after,'rust'),expected)
        for changed in [after.replace(b'1.25',b'1.26'),after.splitlines(keepends=True)[0],b''.join(reversed(after.splitlines(keepends=True)))]:
            self.assertNotEqual(m.stream(changed,'rust'),expected)
        for changed in [after.replace(file.encode(),b'fictional.rs'),after.replace(f':{line}]'.encode(),b':1]'),before]:
            with self.assertRaises(AssertionError):m.stream(changed,'rust')
        data={'stdout':after.decode(),'model.par':after.decode(),'value':1.25}
        projected=m.structured(data,'rust')
        self.assertEqual(projected['stdout'].encode(),expected)
        self.assertEqual(projected['model.par'],data['model.par'])
        self.assertEqual(projected['value'],1.25)

    def test_version_identity_counterpart_and_site(self):
        m=self.mapping;(file,line),(native,function)=next(iter(m.versions.items()))
        row=f'[TEMPO2-RUST VERSION:] [{file}:{line}] [TEMPO2 counterpart: {native}::{function}]\n'.encode()
        old=f'[TEMPO2 VERSION:] [tempo2.h            ] [                    ] [2026.09.1           ]\n[TEMPO2 VERSION:] [{native:<20.20}] [{function:<20.20}] [$Id$                ]\n'.encode()
        self.assertEqual(m.stream(m.identity+row,'rust'),m.stream(old,'native'))
        for changed in [row.replace(file.encode(),b'wrong.rs'),row.replace(f':{line}]'.encode(),b':1]'),row.replace(native.encode(),b'wrong.C'),m.identity.replace(b'0.1.0',b'9.9.9')]:
            with self.assertRaises(AssertionError):m.stream(changed,'rust')

    def test_telescope_unused_branches_keep_explicit_source_sites_and_bodies(self):
        m=self.mapping
        lines=(593,596,599,602,605,608)
        (file,line),_=next((site,value) for site,value in m.sites.items()
                          if site[0]=='source_par.rs' and value==('readParfile.C',lines))
        for native,key in zip(lines,['TEL_VX','TEL_VY','TEL_VZ','TEL_X0','TEL_Y0','TEL_Z0'],strict=True):
            body=f" parameter '{key}' not used\n".encode()
            expected=m.stream(f'[readParfile.C:{native}]'.encode()+body,'native')
            actual=f'[{file}:{line}]'.encode()+body
            self.assertEqual(m.stream(actual,'rust'),expected)
            self.assertNotEqual(m.stream(actual.replace(b'not used',b'used'),'rust'),expected)
        with self.assertRaises(AssertionError):
            m.stream(b"[source_par.rs:1] parameter 'TEL_VX' not used\n",'rust')

    def test_long_line_is_not_repaired_or_dropped(self):
        m=self.mapping;(file,line),native=next((site,value) for site,value in m.sites.items() if site[0]=='general_fitting.rs')
        before=(f'[{native[0]}:{native[1][0]}] '.encode()+b'x'*400+b'\n')[:359]
        after=(f'[{file}:{line}] '.encode()+b'x'*400+b'\n')[:359]
        self.assertNotEqual(m.stream(before,'native'),m.stream(after,'rust'))
        self.assertEqual(m.stream(after,'rust').count(b'x'),after.count(b'x'))
        self.assertFalse(m.stream(after,'rust').endswith(b'\n'))

    def test_internal_comparison_tags_cannot_enter_as_raw_output(self):
        m=self.mapping
        (file,line),native=next((site,value) for site,value in m.sites.items() if site[0]=='general_fitting.rs')
        raw=f'[{file}:{line}] value=1.25\n'.encode()
        changed=raw.replace(f'[{file}:{line}]'.encode(),m.native[native[0],native[1][0]])
        (file,line),(native,function)=next(iter(m.versions.items()))
        row=f'[TEMPO2-RUST VERSION:] [{file}:{line}] [TEMPO2 counterpart: {native}::{function}]\n'.encode()
        internal_row=f'[TEMPO2 SOURCE VERSION:] {native}::{function}\n'.encode()
        # These substitutions previously compared equal to genuine old output.
        for altered in [changed,b'\x1b[33m'+changed,m.header+row,m.identity+internal_row]:
            for kind in ['rust','native']:
                with self.subTest(kind=kind,altered=altered):
                    with self.assertRaisesRegex(AssertionError,'internal comparison label'):
                        m.stream(altered,kind)
        # A native filename or tag inside message data remains ordinary data.
        body=b' literal [TEMPO2:t2fit.C:850] [TEMPO2 SOURCE VERSION:]\n'
        self.assertTrue(m.stream(raw.rstrip()+body,'rust').endswith(body))

    def test_generator_successor_changes_only_location_arguments(self):
        source='d.log(b\'w\', "textOutput.C", 1847, &format("value %g", &[x.into()])?);\n'
        result,count=attribute(source)
        self.assertEqual(count,1)
        self.assertIn('d.rust_log(/* TEMPO2 textOutput.C:1847 */ b\'w\', &format("value %g", &[x.into()])?);',result)
        with self.assertRaises(AssertionError):attribute(result)
        with self.assertRaises(AssertionError):attribute(source+'d.log(b\'m\', "unknown.C", 1, b"unknown");')




class SourceLogBytes(unittest.TestCase):
    def test_redirected_parser_uses_source_bytes_not_ansi_grammar(self):
        from diagnostic_attribution import redirected_log
        self.assertEqual(redirected_log(b'a\x1bnot-an-ANSI-codemZ'),b'aZ')
        self.assertEqual(redirected_log(b'a\x1bunfinished'),b'a')
        self.assertEqual(redirected_log(b'a\x1bunterminated'),b'ainated')
        self.assertEqual(redirected_log(b'a\0ignored\x1bmZ'),b'a')
        self.assertEqual(redirected_log(b'm@END 4 alias\n\xff'),b'm@END 4 alias\n\xff')
        self.assertEqual(redirected_log(b'x'*358+b'\x1b[0mTAIL'),b'x'*358)
        self.assertEqual(redirected_log(b'x'*360),b'x'*359)

    def test_complete_backing_boundary_and_colored_cap(self):
        from diagnostic_attribution import diagnostic_rendering
        for severity,overhead in [('warning',23),('error',15)]:
            wire,visible=diagnostic_rendering(b'[observatory.rs:1]',b'x'*358,severity)
            self.assertEqual(len(visible),359-overhead)
            self.assertGreater(len(wire),359)
            self.assertNotIn(b'\x1b',visible)
        for message in [b'x'*359,b'x\0hidden']:
            with self.assertRaises(AssertionError):diagnostic_rendering(b'[observatory.rs:1]',message,'warning')
        with self.assertRaises(AssertionError):diagnostic_rendering(b'[observatory.rs:1]',b'x','debug')


# Saved producer products are optional for clean pre-bootstrap unit controls.
# The explicit 0116 replay supplies the immutable cap receipt and a fresh output.
import os
@unittest.skipUnless(os.environ.get('AL_RENDERING_EVIDENCE'),'requires the retained 0112 cap evidence')
class SavedTerrestrialRendering(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        import gzip,hashlib,importlib.util,json,shutil,sys
        from corpus_compare import stage_terrestrial_comparison
        cls.gzip=gzip;cls.json=json;cls.shutil=shutil
        cls.digest=staticmethod(lambda data:hashlib.sha256(data).hexdigest())
        receipt=Path(os.environ['AL_RENDERING_EVIDENCE']).resolve()
        assert cls.digest(receipt.read_bytes())=='d68ec56eb523df4679f610681ce478aa747fb7f3a406fa7a0731fa443a1d006b'
        cls.binding=json.loads(receipt.read_text())
        for name,h in cls.binding['inputs'].items():assert cls.digest(Path(name).read_bytes())==h,name
        cls.source=Path(next(name for name in cls.binding['inputs'] if name.endswith('/validation/terrestrial_resources_port/run.py'))).parents[2]
        cls.original=cls.source/'rust/verification/terrestrial_resources'
        cls.catalog=json.loads((cls.source/'validation/terrestrial_resources/outputs/frozen/cases.json').read_text())
        cls.oracle=json.loads(gzip.decompress((cls.original/'corrected_native/reference.json.gz').read_bytes()))
        cls.output=Path(os.environ['AL_RENDERING_OUTPUT']).resolve()
        assert cls.output.is_relative_to(ROOT/'validation/release/build')
        cls.output.mkdir(parents=True,exist_ok=False);cls.rows=[];cls.serial=0
        stage=cls.output/'staged';port=stage/'validation/terrestrial_resources_port';port.mkdir(parents=True)
        (stage/'validation/release').mkdir();(stage/'rust').mkdir()
        (stage/'rust/src').symlink_to(ROOT/'rust/src',target_is_directory=True)
        shutil.copyfile(ROOT/'rust/Cargo.toml',stage/'rust/Cargo.toml')
        shutil.copyfile(ROOT/'validation/release/diagnostic_attribution.py',stage/'validation/release/diagnostic_attribution.py')
        for name in ['run.py','controls.py','freeze.py']:shutil.copyfile(ROOT/'validation/terrestrial_resources_port'/name,port/name)
        cls.adaptations={}
        def write(path,text):
            before=cls.digest(path.read_bytes());path.write_text(text)
            cls.adaptations[str(path.relative_to(stage))]={'original':before,'adapted':cls.digest(path.read_bytes())}
        stage_terrestrial_comparison(stage,write)
        # Import the actual staged runner; no main/build/execute function runs.
        previous=sys.modules.pop('freeze',None);sys.path.insert(0,str(port));sys.dont_write_bytecode=True
        try:
            spec=importlib.util.spec_from_file_location('al_saved_w_run',port/'run.py')
            cls.runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(cls.runner)
        finally:
            sys.path.remove(str(port));sys.modules.pop('freeze',None)
            if previous is not None:sys.modules['freeze']=previous
        assert cls.runner.compare_terrestrial.__code__.co_filename.endswith('diagnostic_attribution.py')
        assert 'try:compare_case(case,expected,actual,' in (port/'run.py').read_text()
        assert 'run.compare_case(case,' in (port/'controls.py').read_text()
        (cls.output/'staging.json').write_text(json.dumps(cls.adaptations,indent=2)+'\n')

    @classmethod
    def tearDownClass(cls):
        for name,h in cls.binding['inputs'].items():assert cls.digest(Path(name).read_bytes())==h,name
        (cls.output/'summary.json').write_text(cls.json.dumps({'scope':'Saved-data replay and mutations; no producer execution',
            'original_cap_receipt':'d68ec56eb523df4679f610681ce478aa747fb7f3a406fa7a0731fa443a1d006b',
            'original_inputs_unchanged':cls.binding['inputs'],'staging':cls.adaptations,'controls':cls.rows,
            'recipes':{name:cls.digest((ROOT/'validation/release'/name).read_bytes()) for name in
                ['diagnostic_attribution.py','corpus_compare.py','test_diagnostic_attribution.py']}},indent=2)+'\n')

    def fixture(self,name='alias_bounded_tokens'):
        import copy
        cls=type(self);cls.serial+=1;folder=self.output/(f'{cls.serial:03d}-'+self._testMethodName)
        base=folder/'evidence';dest=folder/'rust';(base/'corrected_native/raw').mkdir(parents=True)
        for file in ['contract.json','reference.json.gz','receipts.json.gz']:
            self.shutil.copyfile(self.original/'corrected_native'/file,base/'corrected_native'/file)
        for suffix in ['stdout','stderr','jsonl.gz']:
            self.shutil.copyfile(self.original/'corrected_native/raw'/(name+'.'+suffix),base/'corrected_native/raw'/(name+'.'+suffix))
        self.shutil.copytree(self.original/'release'/name,dest)
        return {'case':copy.deepcopy(next(c for c in self.catalog if c['id']==name)),
                'expected':copy.deepcopy(next(c for c in self.oracle if c['id']==name)),
                'actual':self.json.loads((dest/'result.json').read_text()),'base':base,'dest':dest}

    def replay(self,f,name,rejected=False,stage=None):
        self.save(f['dest']/'result.json',f['actual'])
        self.runner.OUT=f['base']
        if rejected:
            with self.assertRaises(AssertionError):self.runner.compare_case(f['case'],f['expected'],f['actual'],f['dest'])
        else:self.runner.compare_case(f['case'],f['expected'],f['actual'],f['dest'])
        report=self.json.loads((f['dest']/'diagnostic-rendering.json').read_text())
        self.assertEqual(report['status'],'failure' if rejected else 'pass')
        if rejected:self.assertTrue(report.get('error'),'rejection needs a concrete reason')
        if stage is not None:self.assertEqual(report['stage'],stage,report.get('error'))
        self.rows.append({'name':name,'rejected':rejected,'report':str((f['dest']/'diagnostic-rendering.json').relative_to(self.output)),
                          'stage':report['stage'],'error':report.get('error'),'result':report.get('result')})
        return report

    def save(self,path,value):path.write_text(self.json.dumps(value,indent=2)+'\n')

    def normalize(self,data):
        return data.decode('latin1').replace(str(self.original/'xxxx'),'{cases}').replace(str(self.original/'fixed00/source'),'{source}').replace(str(self.source/'validation/terrestrial_resources'),'{terrestrial}')

    def rust_stream(self,f,data,channel='stdout'):
        p=f['dest']/('streams.'+channel);p.write_bytes(data)
        receipt=self.json.loads((f['dest']/'receipt.json').read_text());receipt['raw'][p.name]=self.digest(data)
        self.save(f['dest']/'receipt.json',receipt);f['actual'][channel]=self.normalize(data)

    def events(self,f,kind,edit):
        name=f['case']['id'];native=kind=='native'
        path=f['base']/'corrected_native/raw'/(name+'.jsonl.gz') if native else f['dest']/'events.jsonl'
        rows=[self.json.loads(line) for line in (self.gzip.decompress(path.read_bytes()) if native else path.read_bytes()).splitlines()]
        edit(rows)
        raw=('\n'.join(self.json.dumps(row) for row in rows)+'\n').encode()
        path.write_bytes(self.gzip.compress(raw,mtime=0) if native else raw)
        value=f['expected'] if native else f['actual'];value['events']=self.json.loads(self.normalize(self.json.dumps(rows if native else rows[1:]).encode()))
        if native:
            self.seal_native(f,'events',raw)
        else:
            receipt=self.json.loads((f['dest']/'receipt.json').read_text());receipt['raw']['events.jsonl']=self.digest(raw)
            self.save(f['dest']/'receipt.json',receipt)

    def seal_native(self,f,channel,raw):
        """Keep copied native raw/result/receipt identities coherent for semantics."""
        name=f['case']['id'];p=f['base']/'corrected_native/receipts.json.gz'
        receipts=self.json.loads(self.gzip.decompress(p.read_bytes()))
        receipt=next(r for r in receipts if r['id']==name);receipt['raw_sha256'][channel]=self.digest(raw)
        if channel=='events':receipt['resources_after']['events.jsonl']={'size':len(raw),'sha256':self.digest(raw)}
        p.write_bytes(self.gzip.compress(self.json.dumps(receipts).encode(),mtime=0))
        oracle=[f['expected'] if r['id']==name else r for r in self.oracle]
        p=f['base']/'corrected_native/reference.json.gz';p.write_bytes(self.gzip.compress(self.json.dumps(oracle).encode(),mtime=0))
        contract=self.json.loads((f['base']/'corrected_native/contract.json').read_text());contract['corrected_sha256']=self.digest(p.read_bytes())
        self.save(f['base']/'corrected_native/contract.json',contract)

    def test_saved_pairs_through_staged_runner(self):
        for name,length in [('alias_bounded_tokens',336),('observatory_exit_read_alias',344),('observatory_missing_alias',344)]:
            with self.subTest(case=name):
                r=self.replay(self.fixture(name),name)
                self.assertEqual(r['result'],'verified source-attribution rendering')
                self.assertIsNotNone(r['strict_difference'])
                self.assertEqual(r['renderings']['native']['emission_bytes'],length)
                for row in r['renderings'].values():self.assertEqual(row['expected_sha256'],row['observed_sha256'])

    def test_non_rendering_fields_remain_exact(self):
        changes={
            'missing_field':lambda r:r.pop('classification'),
            'case_membership':lambda r:r.__setitem__('id','another_case'),
            'classification':lambda r:r.__setitem__('classification','changed'),
            'action_order':lambda r:r['actions'].reverse(),
            'event_order':lambda r:r['events'].reverse(),
            'missing_event':lambda r:r['events'].pop(),
            'counter':lambda r:r['events'][-1]['state'].__setitem__('warnings',2),
            'boolean_counter':lambda r:r['events'][-1]['state'].__setitem__('warnings',True),
            'numeric_state':lambda r:r['events'][-1]['state'].__setitem__('observatories_initialized',2),
            'backing_message':lambda r:r['events'][-1]['state']['warning_log'].__setitem__(0,'changed'),
            'outcome':lambda r:r.__setitem__('status',1),
            'stderr':lambda r:r.__setitem__('stderr',r['stderr']+'extra'),
        }
        for name,edit in changes.items():
            with self.subTest(mutation=name):
                f=self.fixture();edit(f['actual']);self.replay(f,name,True,'non-rendering')

    def test_rendered_and_surrounding_bytes_rejected_after_rebinding(self):
        prefix=b'[observatory.rs:334]';native_prefix=b'['+str(self.original/'fixed00/source/observatory.C').encode()+b':272]'
        changes={
            'hidden_rust_tail':lambda s:s.replace(b"CCC'\n",b"CCZ'\n"),
            'numeric_message_token':lambda s:s.replace(b'xxx',b'x1x',1),
            'severity':lambda s:s.replace(b'Warning:',b'ERROR:'),
            'wrong_file':lambda s:s.replace(b'observatory.rs',b'unknown.rs'),
            'wrong_site':lambda s:s.replace(b':334]',b':181]'),
            'native_label':lambda s:s.replace(prefix,native_prefix),
            'internal_label':lambda s:s.replace(prefix,b'[TEMPO2:observatory.C:272]'),
            'dropped_newline':lambda s:s.replace(b"'\n@END",b"'@END"),
            'extra_newline':lambda s:s.replace(b"'\n@END",b"'\n\n@END"),
            'missing_diagnostic':lambda s:s[:s.index(prefix)]+b'@END 4 alias\n',
            'common_prefix_only':lambda s:s[:s.index(prefix)]+prefix+(self.original/'corrected_native/raw/alias_bounded_tokens.stdout').read_bytes().split(native_prefix)[1].removesuffix(b'@END 4 alias\n')+b'@END 4 alias\n',
            'outside_bytes':lambda s:s.replace(b'@BEGIN 0 profile',b'@BEGIN 9 profile'),
            'extra_marker_like_text':lambda s:s+b'@END 4 alias\n',
        }
        for name,edit in changes.items():
            with self.subTest(mutation=name):
                f=self.fixture();raw=(f['dest']/'streams.stdout').read_bytes();altered=edit(raw);self.assertNotEqual(raw,altered)
                self.rust_stream(f,altered)
                stage='attribution' if name in ['wrong_file','native_label','internal_label'] else 'rendering'
                self.replay(f,name,True,stage)
        f=self.fixture('observatory_exit_read_alias');data=(f['dest']/'streams.stdout').read_bytes()
        self.rust_stream(f,data.replace(b'/absent',b'/absent2'));self.replay(f,'changed_filename',True,'rendering')
        f=self.fixture();data=(f['dest']/'streams.stdout').read_bytes();self.rust_stream(f,b'',channel='stdout')
        self.rust_stream(f,(f['dest']/'streams.stderr').read_bytes()+data,channel='stderr');self.replay(f,'changed_channel',True,'non-rendering')
        f=self.fixture();path=f['base']/'corrected_native/raw/alias_bounded_tokens.stdout'
        data=path.read_bytes().replace(b'observatory.C:272]',b'observatory.C:163]');path.write_bytes(data)
        f['expected']['stdout']=self.normalize(data);self.seal_native(f,'stdout',data)
        self.replay(f,'native_emitter_site',True,'rendering')

    def test_incomplete_and_ambiguous_backing_records(self):
        changes={
            'missing_backing':lambda rows:rows[-1]['state'].__setitem__('warning_log',[]),
            'truncated_backing':lambda rows:rows[-1]['state'].__setitem__('warning_log',['x'*359]),
            'nul_backing':lambda rows:rows[-1]['state'].__setitem__('warning_log',['x\0hidden']),
            'ambiguous_storage':lambda rows:rows[-1]['state']['warning_log'].append('second'),
            'ambiguous_counter':lambda rows:rows[-1]['state'].__setitem__('warnings',2),
            'wrong_event_association':lambda rows:rows[-1].__setitem__('event',0),
            'wrong_operation_association':lambda rows:rows[-1].__setitem__('operation','read_alias'),
            'unexplained_matched_messages':lambda rows:rows[-1]['state'].__setitem__('warning_log',['same invented message']),
        }
        for name,edit in changes.items():
            with self.subTest(mutation=name):
                f=self.fixture()
                for kind in ['native','rust']:self.events(f,kind,edit)
                stage='backing-message' if name in ['truncated_backing','nul_backing','unexplained_matched_messages'] else 'event-association'
                self.replay(f,name,True,stage)

    def test_raw_input_and_source_binding(self):
        for label,file in [('missing_raw_events','events.jsonl'),('missing_independent_receipt','receipt.json'),('missing_input','input.json')]:
            f=self.fixture();(f['dest']/file).unlink();self.replay(f,label,True,'evidence-integrity')
        f=self.fixture();(f['dest']/'streams.stdout').write_bytes(b'changed without receipt');self.replay(f,'raw_identity',True,'evidence-integrity')
        for name,edit in {
            'wrong_source_identity':lambda r:r['identity']['sources'].__setitem__('rust/src/observatory.rs','changed'),
            'changed_file_membership':lambda r:r['resources_after'].pop(next(iter(r['resources_after']))),
            'changed_input_identity':lambda r:r.__setitem__('input_sha256','changed'),
            'changed_outcome_identity':lambda r:r.__setitem__('library_outcome','source_exit'),
        }.items():
            f=self.fixture();path=f['dest']/'receipt.json';r=self.json.loads(path.read_text());edit(r);self.save(path,r);self.replay(f,name,True,'evidence-integrity')
        for name,edit in {
            'native_formatter_identity':lambda p:p['dependencies'].__setitem__(next(k for k in p['dependencies'] if k.endswith('/TKlog.C')),'changed'),
            'unqualified_logger_flags':lambda p:p['flags'].append('-DLOG_OUTFILE=stderr'),
            'native_executable_identity':lambda p:p.__setitem__('executable_sha256','changed'),
        }.items():
            f=self.fixture();path=f['base']/'corrected_native/contract.json';r=self.json.loads(path.read_text());edit(r['profile']);self.save(path,r);self.replay(f,name,True,'evidence-integrity')

    def test_strict_comparison_and_complete_catalogue_preserved(self):
        f=self.fixture('eop_first_use_cache');r=self.replay(f,'unchanged_strict_case')
        self.assertEqual(r['result'],'strict attribution comparison');self.assertIsNone(r['strict_difference'])
        identifiers=[c['id'] for c in self.catalog]
        self.runner.compare(identifiers,[r['id'] for r in self.oracle],'full original catalogue')
        self.assertEqual(len(identifiers),130)
        for changed in [identifiers[:-1],list(reversed(identifiers))]:
            with self.assertRaises(AssertionError):self.runner.compare(identifiers,changed,'case membership')
        with self.assertRaises(AssertionError):
            from corpus_compare import stage_terrestrial_comparison
            stage_terrestrial_comparison(self.output/'staged',lambda path,text:path.write_text(text))


if __name__=='__main__':unittest.main()
