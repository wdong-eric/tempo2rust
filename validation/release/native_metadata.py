"""0109 native caller and host metadata contracts; scientific values stay exact."""
import hashlib
from pathlib import Path
import re
import signal


def cheby_assertion(summary, status):
    """Recognize only the measured GNU max-Cheby assertion termination."""
    match=re.fullmatch(r"[^:\n]+: (?:[^\n]*[/])?jpleph\.c:326: [^\n]+: Assertion `ncf < MAX_CHEBY' failed\.",summary)
    assert match and status==-signal.SIGABRT,('unexpected GNU Cheby failure',status,summary)
    return ['ncf < MAX_CHEBY']


def assertion_emitter(text, executable, status):
    """Separate the identity-bound executable label from GNU assertion text."""
    pattern=r"(?m)^([^:\n]+): ([^\n]+: Assertion `[^\n]+' failed\.)$"
    def replace(match):
        assert match[1] in [str(executable),Path(executable).name],('unexpected assertion emitter',match[1])
        assert status==-signal.SIGABRT,('assertion did not abort',status)
        return '{executable}: '+match[2]
    return re.sub(pattern,replace,text)


def loader_identity(lines):
    """Retain library order, names, paths and bytes; discard only load addresses."""
    result=[]
    for line in lines:
        match=re.fullmatch(r'\s*(.*?) \(0x[0-9a-fA-F]+\)',line)
        assert match,('unrecognized ldd record',line)
        label=match[1]
        if label=='linux-vdso.so.1':
            result.append({'name':label,'path':None,'sha256':None})
            continue
        if ' => ' in label:name,path=label.split(' => ',1)
        else:path=label;name=Path(path).name
        assert Path(path).is_absolute() and Path(path).is_file(),('unresolved loaded library',line)
        result.append({'name':name,'path':path,'sha256':hashlib.sha256(Path(path).read_bytes()).hexdigest()})
    assert result,'empty loader measurement'
    return result


def precision_profile(macros):
    """Use the compiled host ABI while retaining the timing binary128 contract."""
    names=['__SIZEOF_DOUBLE__','__DBL_MANT_DIG__','__SIZEOF_LONG_DOUBLE__','__LDBL_MANT_DIG__',
           '__FLT128_MANT_DIG__']
    measured=[]
    for name in names:
        match=re.search(r'^#define '+name+r' (\d+)$',macros,re.M)
        assert match,('missing compiler precision macro',name)
        measured.append(int(match[1]))
    # GCC16/arm64 has no __SIZEOF_FLOAT128__ macro. The actual probe must still
    # report the required 16-byte timing storage alongside 113 significand bits.
    values=[*measured[:4],16,measured[4]]
    assert values[:2]==[8,53] and values[4:]==[16,113],values
    assert values[2:4] in [[8,53],[16,64]],('unqualified ordinary ABI',values)
    return values


def record_adaptation(adaptations, name, before, after):
    """Compose observed staging edits, requiring a continuous identity chain."""
    if name in adaptations:
        record=adaptations[name]
        assert record['adapted']==before,('broken adaptation chain',name)
        if before!=after:
            record.setdefault('intermediates',[]).append(before)
            record['adapted']=after
    elif before!=after:
        adaptations[name]={'original':before,'adapted':after}


def inherit_adaptations(root, native):
    """Bind preparation edits to their copied originals and current staged bytes."""
    adaptations={}
    for name,record in native['source_adaptations'].items():
        assert record['original']==native['sources'][name],('adaptation source differs',name)
        current=hashlib.sha256((root/name).read_bytes()).hexdigest()
        assert current==record['adapted'],('adapted source differs',name)
        adaptations[name]={**record,'intermediates':list(record.get('intermediates',[]))}
    return adaptations


def preservation_expected(original, adaptations):
    """Bind named staging edits without rewriting the historical manifest."""
    expected=dict(original)
    for name,record in adaptations.items():
        if name in expected:
            # A native freeze may precede or follow a preparation edit. Accept
            # only an identity observed in the continuous preparation/port chain.
            assert expected[name] in [record['original'],*record.get('intermediates',[]),record['adapted']],('adaptation identity differs from freeze',name)
            expected[name]=record['adapted']
    return expected
