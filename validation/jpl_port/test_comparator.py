"""I-only negative controls for exact coverage, numerical state and execution."""
import copy
import tempfile
import unittest
from pathlib import Path
import run as port

class ProjectionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.ref=port.load(port.H/'outputs/reference.json')
        cls.rules=port.load(port.HERE/'projection.json')
        cls.good={name:[{k:e[k] for k in rule['exact_fields']} for e,rule in zip(events,cls.rules['scenarios'][name],strict=True)] for name,events in cls.ref.items()}
        cls.good['title_zero']=cls.good['title_zero'][:1]+[dict(event=0,op='open',boundary='title_conversion',init_error=0,io_calls=[1,2,0],last_fread_size_requested_returned=[204,1,1])]

    def reject(self,mutate):
        a=copy.deepcopy(self.good);mutate(a)
        with self.assertRaises(AssertionError):port.compare(a,self.ref,self.rules)

    def test_complete_control(self):
        self.assertEqual(port.compare(self.good,self.ref,self.rules)['reference_events'],1443)

    def test_mutations(self):
        def change_number(a,case,event,field,index,scale):
            n=a[case][event][field][index];x=float(n['decimal'])*scale
            n['decimal']=str(x);n['bits']=port.struct.pack('>d',x).hex()
        cases={
            'title_deleted':lambda a:a.pop('title_zero'),
            'title_wrong_boundary':lambda a:a['title_zero'][1].update(boundary='read_error'),
            'title_fabricated_success':lambda a:a['title_zero'].__setitem__(1,copy.deepcopy(self.ref['title_zero'][1])),
            'title_fabricated_unreachable_execution':lambda a:a['title_zero'].append(copy.deepcopy(self.ref['title_zero'][2])),
            'missing_scenario':lambda a:a.pop('zero_au'),
            'extra_scenario':lambda a:a.update(extra=[]),
            'missing_event':lambda a:a['zero_au'].pop(),
            'extra_event':lambda a:a['zero_au'].append(a['zero_au'][-1]),
            'wrong_event_order':lambda a:a['zero_au'].reverse(),
            'missing_field':lambda a:a['zero_au'][1].pop('ncon'),
            'extra_field':lambda a:a['zero_au'][1].update(units='km'),
            'wrong_index':lambda a:a['zero_au'][1].update(event=99),
            'wrong_status':lambda a:a['partial_far_137_le'][5].update(status=0),
            'wrong_class':lambda a:a['zero_au'][3]['sun'][0].update({'class':'NaN','decimal':'NaN'}),
            'missing_component':lambda a:a['state_sequences_le'][3]['pv'].pop(),
            'extra_component':lambda a:a['state_sequences_le'][3]['pv'].append(a['state_sequences_le'][3]['pv'][-1]),
            'wrong_units_scale':lambda a:change_number(a,'state_sequences_le',3,'pv',0,86400),
            'wrong_value':lambda a:change_number(a,'state_sequences_le',3,'pv',0,2),
            'cache_commit_after_success':lambda a:a['partial_far_137_le'][5].update(cache_loc=a['partial_far_137_le'][3]['cache_loc']),
            'discard_partial_bytes':lambda a:a['partial_far_137_le'][5].update(cache_bytes=a['partial_far_137_le'][3]['cache_bytes']),
            'changed_failure_disposition':lambda a:a['de405_truncated_pleph'][4].update(out=[]),
            'missing_numeric_field':lambda a:a['state_sequences_le'][3]['pv'][0].pop('bits'),
            'extra_numeric_field':lambda a:a['state_sequences_le'][3]['pv'][0].update(error=0),
            'wrong_fread_count':lambda a:a['partial_far_137_le'][5].update(last_fread_size_requested_returned=[8,825,825]),
        }
        for name,mutate in cases.items():
            with self.subTest(name=name):self.reject(mutate)


    def test_projection_classification_controls(self):
        for mode in ['delete_unreachable','reorder_unreachable','wrong_unreachable','unapproved_elsewhere','missing_reference_unreachable']:
            with self.subTest(mode=mode):
                rules=copy.deepcopy(self.rules);ref=copy.deepcopy(self.ref)
                if mode=='delete_unreachable':rules['scenarios']['title_zero'].pop(3)
                if mode=='reorder_unreachable':rules['scenarios']['title_zero'][2],rules['scenarios']['title_zero'][3]=rules['scenarios']['title_zero'][3],rules['scenarios']['title_zero'][2]
                if mode=='wrong_unreachable':rules['scenarios']['title_zero'][2]['mode']='exact'
                if mode=='unapproved_elsewhere':rules['scenarios']['zero_au'][1]['mode']='unreachable'
                if mode=='missing_reference_unreachable':ref['title_zero'].pop(3)
                with self.assertRaises(AssertionError):port.compare(self.good,ref,rules)

class ExecutionTests(unittest.TestCase):
    def test_success_with_stale_output_and_no_probe_is_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            inputs=Path(d)/'inputs';inputs.mkdir()
            observed=inputs.parent/'observed.json';observed.write_text('{"stale_control":true}')
            with self.assertRaises(AssertionError):port.execute(Path('/usr/bin/true'),{'commands':['never executed']},inputs,'stale')
            self.assertFalse(observed.exists())

    def test_zero_selected_tests_with_output_is_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            inputs=Path(d)/'inputs';inputs.mkdir()
            fake=Path(d)/'zero-tests'
            fake.write_text('#!/usr/bin/env python3\nimport os\nopen(os.environ["JPL_OUTPUT"],"w").write("[]")\nprint("running 0 tests\\ntest result: ok. 0 passed; 0 failed")\n')
            fake.chmod(0o755)
            with self.assertRaises(AssertionError):port.execute(fake,{'commands':[]},inputs,'zero-tests')

    def test_claimed_probe_without_output_is_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            inputs=Path(d)/'inputs';inputs.mkdir()
            (Path(d)/'observed.json').write_text('{"stale_control":true}')
            fake=Path(d)/'missing-output'
            fake.write_text('#!/bin/sh\necho "test jpleph::tests::probe ... ok"\necho "test result: ok. 1 passed; 0 failed"\n')
            fake.chmod(0o755)
            with self.assertRaisesRegex(AssertionError,'missing fresh'):port.execute(fake,{'commands':[]},inputs,'missing')


class IsolatedTests(unittest.TestCase):
    def test_changed_boundary_disposition_rejected(self):
        observed=port.load(port.OUT/'debug_isolated_raw.json')
        specs=port.load(port.H/'outputs/isolated.json')['cases']
        rules=port.load(port.HERE/'isolated_projection.json')
        self.assertEqual(len(port.isolated_compare(observed,specs,rules)),10)
        observed['nan_epoch'][-1]['boundary']='source_status_minus_1'
        with self.assertRaises(AssertionError):port.isolated_compare(observed,specs,rules)

if __name__=='__main__':unittest.main()
