"""Positive controls and artifact mutations of the actual L acceptance gates."""
import copy
import struct
import unittest
import run as port


class ComparisonControls(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.refs={c['id']:port.expected_record(c) for c in port.load(port.J/'outputs/reference.json')['cases']}
        cls.delivery=port.load(port.OUT/'debug_direct.json')

    def rejected(self,case,edit):
        original=self.refs[case];changed=copy.deepcopy(original);edit(changed)
        with self.assertRaises(AssertionError):port.compare(changed,original)

    def test_positive_reference_and_complete_delivery(self):
        for case in self.refs.values():port.compare(case,copy.deepcopy(case))
        port.validate_delivery(self.delivery)

    def test_missing_extra_reordered_cases(self):
        for action in ('missing','extra','reordered'):
            with self.subTest(action=action):
                p=copy.deepcopy(self.delivery)
                if action=='missing':p['results'].pop()
                elif action=='extra':p['results'].append(p['results'][0])
                else:p['results'][0],p['results'][1]=p['results'][1],p['results'][0]
                with self.assertRaises(AssertionError):port.validate_delivery(p)

    def test_missing_extra_reordered_actions_events_components(self):
        for key in ('actions','events'):
            for action in ('missing','extra','reordered'):
                def edit(c):
                    if action=='missing':c[key].pop()
                    elif action=='extra':c[key].append(c[key][0])
                    else:c[key][0],c[key][1]=c[key][1],c[key][0]
                with self.subTest(key=key,action=action):self.rejected('route_simple',edit)
        for action in ('missing','extra','reordered'):
            def edit(c):
                items=c['events'][-1]['table_queries']
                if action=='missing':items.pop()
                elif action=='extra':items.append(items[0])
                else:items.reverse()
            with self.subTest(components=action):self.rejected('sequential_epochs',edit)

    def test_missing_extra_fields(self):
        self.rejected('table_ordinary',lambda c:c['events'][-1]['result'].pop('value'))
        self.rejected('table_ordinary',lambda c:c['events'][-1]['result'].update(extra=0))

    def test_behavior_mutations(self):
        # Each location is an actual initialized field in J's corresponding case.
        edits={
            'permissive parsing':('table_permissive',lambda c:c['events'][3]['result']['table']['samples'].pop()),
            'binary32 weight':('route_binary32_rounding',lambda c:c['events'][-1]['clock_state']['functions'][0]['badness'].update(binary32='4b800001')),
            'strict tie':('route_node_tie',lambda c:c['events'][-1]['clock_state']['sequences'][0]['indices'].reverse()),
            'directed route':('route_simple',lambda c:c['events'][-1]['clock_state']['functions'][0].update(to='A')),
            'fallback':('fallback_both_w0',lambda c:c['events'][-1]['clock_state']['sequences'][0].update(from_key='A')),
            'shifted epoch':('sequential_epochs',lambda c:c['events'][-1]['table_queries'][1].update(epoch=c['events'][-1]['table_queries'][0]['epoch'])),
            'loaded cache lifetime':('lifetime_after_load_remove',lambda c:c['events'][-1]['clock_state']['functions'][0].update(loaded=False)),
            'SAT jump indexing':('toa_duplicate_memberships',lambda c:c['events'][-1]['result']['observations'][0]['after'].update(count=1)),
            'extra shifted slot':('toa_capacity_28',lambda c:c['events'][-1]['result']['observations'][0]['after']['slots'][29].update(destination_bytes='00'*32)),
            'destination change mask':('toa_capacity_28',lambda c:c['events'][-1]['result']['observations'][0]['after']['slots'][29].update(changed_bytes='0'*40)),
            'warning suppression':('lifetime_warning_state',lambda c:c.update(stderr=c['stderr']+'duplicate warning\n')),
            'warning history':('lifetime_warning_state',lambda c:c['events'][-1]['clock_state'].update(warnings=0)),
            'source exit status':('table_missing',lambda c:c.update(returncode=0)),
            'source exit classification':('table_missing',lambda c:c.update(classification='ordinary')),
            'source exit event prefix':('table_missing',lambda c:c['events'].append(dict(event=2,phase='end',operation='load'))),
        }
        for name,(case,edit) in edits.items():
            with self.subTest(mutation=name):self.rejected(case,edit)

    def test_finite_bits_signed_zero_and_nonfinite_classes(self):
        original={'class':'finite','bits':'3ff0000000000000','decimal':'1'}
        changed={'class':'finite','bits':'3ff0000000000001','decimal':repr(struct.unpack('>d',bytes.fromhex('3ff0000000000001'))[0])}
        with self.assertRaises(AssertionError):port.compare(changed,original)
        with self.assertRaises(AssertionError):port.compare(dict(changed,decimal='1'),changed)
        with self.assertRaises(AssertionError):port.compare({'class':'finite','bits':'8000000000000000','decimal':'-0'}, {'class':'finite','bits':'0000000000000000','decimal':'0'})
        with self.assertRaises(AssertionError):port.compare({'class':'nan'},{'class':'+inf'})

    def test_boundary_classification_and_defined_prefix(self):
        cases=port.load(port.J/'cases.json')
        for case_id in port.BOUNDS:
            c=next(c for c in cases if c['id']==case_id)
            raw=next(r['raw'] for r in self.delivery['results'] if r['id']==case_id)
            port.validate_boundary(c,raw)
            for field,value in [('field','source_exit'),('event',999),('reentry','reset')]:
                changed=copy.deepcopy(raw);changed['failure'][field]=value
                with self.subTest(case=case_id,field=field),self.assertRaises(AssertionError):port.validate_boundary(c,changed)
        c=next(c for c in cases if c['id']=='toa_capacity_29')
        raw=copy.deepcopy(next(r['raw'] for r in self.delivery['results'] if r['id']==c['id']))
        raw['failure']['observations'][0]['count']=29
        with self.assertRaises(AssertionError):port.validate_boundary(c,raw)

    def test_wrong_execution_source_input_comparator_and_stale_identity(self):
        expected=dict(case='table_ordinary',token='fresh',source='pinned',input='input sha',executable='binary sha',comparator='runner sha')
        port.validate_identity(expected,copy.deepcopy(expected))
        for field in expected:
            changed=dict(expected);changed[field]='stale or wrong'
            with self.subTest(identity=field),self.assertRaises(AssertionError):port.validate_identity(expected,changed)
        for field in ('run_token','input_sha256','executable_sha256'):
            changed=copy.deepcopy(self.delivery);changed['identities'][0][field]='0'*64
            with self.subTest(output_identity=field),self.assertRaises(AssertionError):port.validate_delivery(changed)


if __name__=='__main__':unittest.main(verbosity=2)
