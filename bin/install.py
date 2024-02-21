#!/usr/bin/python3

import os, sys

if __name__ != "__main__":
    sys.stderr.write('The executable install module must not be imported.')
    sys.exit(1)

SCRIPT_FILE = os.path.abspath(__file__)
BASE_DIR = os.path.dirname(SCRIPT_FILE)
HOME_DIR = os.path.abspath(BASE_DIR + "/..")
sys.path.append(HOME_DIR)

print(f'__file__={__file__}')
print(f'SCRIPT_FILE={SCRIPT_FILE}')
print(f'BASE_DIR={BASE_DIR}')
print(f'HOME_DIR={HOME_DIR}')
print(f'sys.path={sys.path}')
