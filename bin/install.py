#!/usr/bin/python3

import os
import sys
import platform

if __name__ != "__main__":
    sys.stderr.write('The executable install module must not be imported.')
    sys.exit(1)


class SysInfo:
    def __init__(self):
        self.os = platform.system().lower()
        self.arch = platform.machine().lower()
        self.shell = os.getenv('SHELL')
        self.home = os.getenv('HOME')

    def __str__(self):
        return (f'SysInfo =>\n'
                f'os: {self.os}\n'
                f'arch: {self.arch}\n'
                f'shell: {self.shell}\n'
                f'home: {self.home}\n')


def run(*args, exit_code=True, multi_command=False, **kwargs):
    if isinstance(args[0], tuple) or isinstance(args[0], list):
        print('is tuple | list')
    elif isinstance(args[0], str):
        print('is str')
    else:
        print('Unsupported parameter type')

    if multi_command:
        rets = []
        # for arg in args[0]:
        #     rets.append(command(arg, exit_code, **kwargs))
        return rets
    else:
        import subprocess
        ret = subprocess.run('ls', executable="/bin/bash", **kwargs)
        # return command(*args, exit_code, **kwargs)


sys_info = SysInfo()
print(sys_info)
run('', 123)
run((), 123)
run([], 123)
