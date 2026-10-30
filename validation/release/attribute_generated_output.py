#!/usr/bin/env python3
"""0107 successor step for historically generated core_output_source.rs.

Run historical translation only in a disposable source export. Pass its generated
file here and review the fresh output before replacing committed Rust. Cargo uses
committed source; source_audit rejects restored native-labelled Rust log calls.
"""
import argparse
from pathlib import Path
import re

HEADER='//! 0107 attribution successor: validation/release/attribute_generated_output.py.\n'
PATTERN=re.compile(r'\.log\(\s*(b\'[demw]\'),\s*"textOutput.C",\s*(\d+),\s*')


def attribute(source):
    result,count=PATTERN.subn(lambda m:f'.rust_log(/* TEMPO2 textOutput.C:{m[2]} */ {m[1]}, ',source)
    assert count, 'No historical output diagnostics found; use the original generated file'
    assert '.log(' not in result,'Unmapped generated diagnostic; review its source before conversion'
    return HEADER+result,count


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--input',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    a=p.parse_args()
    result,count=attribute(a.input.read_text())
    with a.out.open('x') as output:output.write(result)
    print(count,'source locations moved to comments; compiler captures Rust sites')
