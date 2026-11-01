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
           '__SIZEOF_FLOAT128__','__FLT128_MANT_DIG__']
    values=[int(re.search(r'^#define '+name+r' (\d+)$',macros,re.M)[1]) for name in names]
    assert values[:2]==[8,53] and values[4:]==[16,113],values
    assert values[2:4] in [[8,53],[16,64]],('unqualified ordinary ABI',values)
    return values


def preservation_expected(original, adaptations):
    """Bind named staging edits without rewriting the historical manifest."""
    expected=dict(original)
    for name,record in adaptations.items():
        if name in expected:
            assert expected[name]==record['original'],('adaptation original differs from freeze',name)
            expected[name]=record['adapted']
    return expected
