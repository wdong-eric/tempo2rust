"""Narrow 0053 source corrections. Original source remains immutable.

Each substitution asserts its pinned target; corrections.patch records exact bytes.
Undefined original values are not numeric reference observations.
"""
import re

def corrected(name, source):
    if name == 'DDKmodel.C':
        old='dtdpx = dtdpx - daopx_dpx;'
        assert old in source
        return source.replace(old,'/* AD: dead read of uninitialized dtdpx; no return consumes it. */')
    if name == 'T2model.C':
        old='if (k > psr->param[p].aSize)'
        assert old in source
        source=source.replace(old,'if (k >= psr->param[p].aSize)')
        source=re.sub(r'psr\[p\]\.param\[(param_\w+)\]\.paramSet\[com\]\s*==\s*1',r'(com < psr[p].param[\1].aSize && psr[p].param[\1].paramSet[com]==1)',source)
    if name == 'ELL1model.C':
        old='k<psr[p].ifuncN-1';assert old in source
        source=source.replace(old,'k<psr[p].orbifuncN-1')
    if name in ['T2model.C','ELL1Hmodel.C']:
        # STIG locals have no definition in H4/H3-only modes. Only mode 1 or
        # the explicitly requested STIG derivative consumes these expressions.
        lines=source.splitlines(True)
        for i,line in enumerate(lines):
            if line.lstrip().startswith('fs =') or line.lstrip().startswith('fs='):
                lines[i]='if (mode == 1 || param == param_stig) {\n'+line
            if line.lstrip().startswith('lsc =') or line.lstrip().startswith('lsc='):
                lines[i]=line+'}\n'
            if name == 'ELL1Hmodel.C':
                stripped=line.lstrip()
                if stripped.startswith('brace=') or stripped.startswith('dlogbr='):
                    lines[i]='if (param == param_sini || param == param_m2) { '+line.strip()+' }\n'
                for local,selector in [('Cm2','m2'),('Csi','sini')]:
                    if stripped.startswith(local+' '):
                        lines[i]=f'if (param == param_{selector}) {{ '+line.strip()+' }\n'
        source=''.join(lines)
    return source
