#!/usr/bin/env python3
"""Complete selected definitions, header branches, bounded dispositions and caller layouts."""
import re
from profile import HERE,blob,fields,save,sha
from fixtures import cases


def inventory():
    rows=[];cs=cases()
    for file in ['TKlongdouble.C','tempo2Util.C','TKlongdouble.float128.h']:
        source=blob(file).decode()
        pattern=r'^(?:inline )?(?:std::string|longdouble|double|int|long|void|__float128|dd_real)\s+(\w+)\s*\(([^;{}]*?)\)\s*\{'
        for m in re.finditer(pattern,source,re.M):
            name=m[1];start=source.count('\n',0,m.start())+1;sig=m[0].split('{')[0].strip()
            key='inline_'+name if file.endswith('.h') else name
            if name=='fortran_mod':key+='_'+('128' if 'longdouble' in m[2] else '64')
            active=not (file=='TKlongdouble.C' and name=='pow')
            matching=[c['id'] for c in cs if key in c['functions']]
            # Nested formatter call membership is measured independently by gcov.
            if name=='ld_vsprintf':matching=[c['id'] for c in cs if c['id'].startswith('format_')]
            rows.append(dict(file=file,line=start,key=key,signature=sig,disposition='executed' if active else 'inactive_DDREAL',cases=matching))
    assert len([r for r in rows if r['file']=='TKlongdouble.C' and r['disposition']=='executed'])==6
    assert len([r for r in rows if r['file']=='tempo2Util.C'])==18
    assert len([r for r in rows if r['file'].endswith('.h')])==11
    assert all(r['cases'] for r in rows if r['disposition']=='executed')
    headers={}
    for file in ['TKlongdouble.float128.h','TKlongdouble.ld.h']:
        source=blob(file).decode()
        headers[file]=dict(sha256=sha(source.encode()),macros=[dict(line=i,text=line) for i,line in enumerate(source.splitlines(),1) if line.startswith('#define ')],
            conditionals=[dict(line=i,text=line) for i,line in enumerate(source.splitlines(),1) if re.match(r'#(?:if|else|endif)',line)],
            disposition='selected raw header copied verbatim to TKlongdouble.h' if 'float128' in file else 'inactive alternative header; all branches source-inspected, no alternate executable')
    return dict(functions=rows,headers=headers,caller_fields={kind:fields(kind) for kind in ['pulsar','observation']},
        undefined_unexecuted=[
            dict(operation='ld_vsprintf',source='TKlongdouble.C:96-190',inputs='empty format; trailing percent or missing conversion; unsupported length modifiers/dynamic star; empty formatted output; output/internal format exceeding storage',reason='do/while reads beyond terminator, missing va_arg types/counts, preincrement scans past empty output, unchecked sprintf/memcpy or residual BUFSIZE assumptions; never executed'),
            dict(operation='dms_turn',source='tempo2Util.C:158',inputs='40 or more bytes including absent NUL capacity',reason='strcpy into ln[40]; never executed'),
            dict(operation='turn_hms/turn_dms',source='tempo2Util.C:68-71,105-108',inputs='nonfinite or unrepresentable integer intermediates; insufficient output',reason='invalid float-to-int conversion or sprintf destination overflow; never executed'),
            dict(operation='fortran_mod',source='tempo2Util.C:189,195',inputs='zero divisor, nonfinite quotient or truncated quotient outside int',reason='int conversion undefined; no golden integer. Finite/Infinity quotient zero is executed, subsequent 0*Infinity yields NaN.'),
            dict(operation='fortran_nint/fortran_nlong',source='tempo2Util.C:203-218',inputs='nonfinite or rounded expression whose truncation is outside target type',reason='check x +/- 0.5 after source-type rounding; source input range alone insufficient; never executed'),
            dict(operation='copyParam/copyPSR',source='tempo2Util.C:283-355',inputs='overlapping strcpy, undersized arrays, invalid counts/pointers, uninitialized source cells',reason='invalid memory/caller state, no fabricated output; disjoint fully initialized storage only'),
            dict(operation='displayMsg',source='tempo2Util.C:394',inputs='key length >= 10 including overflow of keyRec[][10]',reason='unchecked strcpy; never executed'),
            dict(operation='getParameterValue',source='tempo2Util.C:428-468',inputs='invalid parameter/array index or missing allocated cells',reason='invalid access; no fabricated value')],
        host_observations=[
            dict(operation='all four formatter full calls',source='TKlongdouble.C:179-180,184-210',reason='va_end(oargs) without va_start; callee va_end(args), then wrapper/caller va_end again; selected ABI observations, never portable defined formatter results'),
            dict(operation='colored TKlog output',source='TKlog.C:60-86',reason='first ESC at buffer[0] causes --ptr to form buffer-1 before ++ptr; no before-buffer dereference in selected strip loop, but not portable C++ pointer arithmetic; host diagnostic observations. No overread/overflow probe.')],
        deferred=['Other platform/ABI/libc/locale profiles','alternate ld header execution and DDREAL backend','TTY color output','TKlog logdbg/logtchk/direct generic logging subsystem beyond displayMsg dependencies','arbitrary formats/fuzzing and known invalid operations','J nine withheld experiments','Rust/API/representation/tolerances and central acceptance'],
        existing_evidence=[dict(owner='J',path='validation/clocks/',acceptance='Decision 0021 accepts executed bounded evidence at 360cc03; nine withheld cases remain open',use='Read-only report/source-map and build-pattern precedent; no J golden rewritten or silently promoted to whole utility coverage'),dict(owner='K',path='validation/parameters/',acceptance='Decision 0026 at 8f92575 accepts corrected 38d62af including ccdf55c within its declared bounded scope; directory read-only',use='Read-only code/build-pattern and controlled parameter snapshot precedent; M independently runs selected utility calls; K acceptance does not establish whole utility coverage')])


if __name__=='__main__':
    inv=inventory();save(HERE/'inventory.json',inv)
    print('Inventoried 6 + 18 primary routines, 11 inline wrappers, both headers and all caller fields')
