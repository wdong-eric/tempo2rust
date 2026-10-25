#!/usr/bin/env python3
"""Record measured native object operations used by localized Rust translations."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BUILD = ROOT / 'validation/input_to_bat/build'
records = []
for relative, addresses, role in [
    ('validation/planetary_ephemeris/build/ordinary/readEphemeris.o',
     [0xb80, 0xb84, 0xc1c, 0xc20, 0xcac, 0xcb0], 'telescope offset intercept and evaluation'),
    ('validation/propagation/build/ordinary/shapiro_delay.o',
     [0x78, 0xa8, 0xac, 0xb0], 'direction update and local norm'),
    ('validation/propagation/build/ordinary/dm_delays.o',
     [0xd4, 0x104, 0x108, 0x10c, 0xb48, 0xbe0, 0xd4c], 'direction norm and spherical solar-wind arithmetic'),
]:
    path = ROOT / relative
    command = ['otool', '-tvV', str(path)]
    assembly = subprocess.check_output(command)
    selected = [line for line in assembly.decode().splitlines() if any(line.startswith(f'{n:016x}\t') for n in addresses)]
    assert len(selected) == len(addresses), relative
    records.append({'object': relative, 'role': role, 'object_sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                    'command': command, 'assembly_sha256': hashlib.sha256(assembly).hexdigest(), 'instructions': selected})
header = ROOT / 'validation/propagation/build/ordinary/source/TKlongdouble.h'
assert '#define powl powq' in header.read_text()
result = {'object_records': records, 'powl_header_sha256': hashlib.sha256(header.read_bytes()).hexdigest(),
          'powl_definition': '#define powl powq',
          'precision': 'Native configured longdouble is binary128/113 bits; Rust production remains Rug128.'}
(BUILD / 'native-numerics.json').write_text(json.dumps(result, indent=2) + '\n')
print('Recorded native object contractions and the actual powl macro')
