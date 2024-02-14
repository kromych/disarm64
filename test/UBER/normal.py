#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys

def _bit_not(num):
    return num ^ ((1 << num.bit_length()) - 1)

def bit_not(n, numbits=32):
    return (1 << numbits) - 1 - n

def try_hex(x):
    if x.startswith('#'):
        num_start = 1
        if x[1] == '-':
            num_start = 2

        suffix = ''
        try:
            if x.endswith(','):
                suffix = ','
            if x.endswith(')'):
                suffix = ')'
            if x.endswith(']'):
                suffix = ']'
            elif x.endswith(']!'):
                suffix = ']!'

            s = x[num_start:len(x)-len(suffix)]
            if '.' in s or 'e+' in s or 'e-' in s:
                f = float(s)
                if f != int(f):
                    if num_start == 2:
                        f = -f
                    return f"#{f:e}{suffix}"
                else:
                    if num_start == 2:
                        f = -f
                    s = str(int(f))
            
            base = 10 if not s.startswith('0x') else 16
            num = int(s, base)

            if num_start == 2:
                num = bit_not(num, 32)

            return f"#0x{num:08x}{suffix}"
        except Exception as e:
            raise e
            # return x
    else:
        return x

with open(sys.argv[1], 'r') as f:
    lines = f.readlines()
    for line in lines:
        if '//' in line:
            line = line.split('//')[0]
        if '<' in line:
            line = line.split('<')[0]
        line = line.lower()

        line = line.strip().split()[1:]
        opcode = line[0]
        mnemonic = line[1]
        operands = ' '.join([try_hex(x) for x in line[2:]]).replace('{ ', '{').replace(' }', '}')

        print(f"{opcode:14}{mnemonic:16}{operands}")

