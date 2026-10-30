"""0104 successor: source decimal writing followed by measured native parsing."""
from decimal import Decimal as D, localcontext
from pathlib import Path
from af_compare import base, NativeChecks, round_binary
import os
from common import Run,sha

historical=base.serialization
raw_records=[]


def native_witness(out):
    run=Run(out);source=Path(__file__).with_name('polyco_write_read.cc')
    run.run([os.environ.get('NATIVE_CXX','g++'),'-std=gnu++11','-O2',source,'-lquadmath','-o',run.out/'witness'],'compile')
    rows=[]
    for line in run.run([run.out/'witness'],'native-write-read').decode().splitlines():
        kind,bits,parsed_bits,value,token,parsed=line.split('\t')
        assert int(parsed_bits)==NativeChecks.bits
        with localcontext() as context:
            context.prec=200
            exact=round_binary(value,int(bits));number=D(exact.numerator)/D(exact.denominator)
            mantissa,exponent=format(number,'.'+('30' if kind=='2' else '17')+'e').split('e')
            expected=mantissa+'e'+format(int(exponent),'+03d')
        assert token==expected,(line,expected)
        assert round_binary(parsed,int(parsed_bits))==round_binary(token,int(parsed_bits)),line
        rows.append(dict(kind=kind,input_bits=bits,parser_bits=parsed_bits,input=value,token=token,parsed=parsed,
                         lossless=round_binary(value,int(parsed_bits))==round_binary(parsed,int(parsed_bits))))
    run.save({'status':'pass','source':sha(source),'records':rows})
    return {'receipt':sha(run.out/'receipt.json'),'records':rows}


def coefficients(path, models):
    lines=Path(path).read_text().splitlines();offset=0;result=[]
    for model in models:
        count=len(model['coeff'])
        if Path(path).name=='polyco_new.dat':
            offset+=2;tokens=[]
            for line in lines[offset:offset+(count+2)//3]:
                tokens.extend(line[i:i+25].strip() for i in range(0,len(line),25))
            offset+=(count+2)//3
        else:
            assert lines[offset]=='TEMPO2: POLYCO TEMPO1 emulation'
            offset+=15;tokens=lines[offset:offset+count];offset+=count
        assert len(tokens)==count,(path,tokens,count)
        result.append(tokens)
    assert offset==len(lines),(path,offset,len(lines))
    return result


def legacy_checks(models, parsed, legacy, modern):
    """The 31-digit companion bounds the pre-write wide value without guessing it.

    A boundary-straddling interval is unavailable evidence, never coerced to a
    binary64 value. This avoids double-rounding through the ordinary model.
    """
    c=NativeChecks()
    for k,(before,after,tokens,wide_tokens) in enumerate(zip(models,parsed,legacy,modern,strict=True)):
        for i,(original,actual,token,wide) in enumerate(zip(before['coeff'],after['coeff'],tokens,wide_tokens,strict=True)):
            label=f'serialized[{k}].coeff[{i}]'
            with localcontext() as context:
                context.prec=200
                value=D(wide)
                if value.is_zero():
                    # Scientific-format zero identifies exact zero, including
                    # its sign; there is no nonzero decimal rounding interval.
                    lo=hi=float(value)
                else:
                    half=D(5).scaleb(value.adjusted()-31)
                    # Direct binary64 conversion includes its exponent limits
                    # and subnormals, avoiding a significand-only intermediate.
                    lo,hi=float(value-half),float(value+half)
            # polyco.C:432 casts timing coeff to double, then %25.17le.
            c.exact(label+'.writer_input_unambiguous',lo,hi)
            if lo!=hi:continue
            c.exact(label+'.source_token',token,format(lo,'.17e'))
            # T1Polyco_Read: ordinary %Lf; do not compare to the pre-write float.
            c.number(label,actual,token)
            # Preserve the pre-serialization ordinary model relationship too.
            c.number(label+'.model_from_wide',original,wide)
    return c


def serialization(kind, original, parsed, path):
    path=Path(path)
    old=historical(kind,original,parsed,path.name)
    raw_records.append({'file':str(path),'checks':old.count,'failures':old.failures})
    if kind!='polyco' or path.name!='polyco_new.dat':return old
    successor=legacy_checks(original,parsed,coefficients(path,original),
                            coefficients(path.with_name('newpolyco.dat'),original))
    # All previous header and count gates remain. Only the explicitly corrected
    # coefficient-lossless predicate is replaced; retain its raw rows separately.
    old.failures=[f for f in old.failures if '.coeff[' not in f.get('field','')]
    old.count-=sum(len(m['coeff']) for m in original)
    old.count+=successor.count;old.failures+=successor.failures
    return old
