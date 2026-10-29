"""Convert the two selected /usr/bin/time profiles to CPU seconds and RSS bytes."""
import re


def parse_usage(text, system):
    if system == 'Darwin':
        match = re.search(r'([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys', text)
        cpu = float(match[2]) + float(match[3])
        rss = int(re.search(r'(\d+)\s+maximum resident set size', text)[1])
    elif system == 'Linux':
        cpu = sum(float(re.search(label + r' time \(seconds\): ([\d.]+)', text)[1])
                  for label in ['User', 'System'])
        rss = 1024 * int(re.search(r'Maximum resident set size \(kbytes\): (\d+)', text)[1])
    else:
        raise ValueError('Unqualified time profile: ' + system)
    return cpu, rss
