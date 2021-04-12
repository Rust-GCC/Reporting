#!/usr/bin/env python3

import sys

def generate_org_line(prnum):
    return "[[https://github.com/Rust-GCC/gccrs/pull/{0}][PR{0}]]".format(prnum)


def main():
    for prnum in sys.argv[1:]:
        line = generate_org_line(prnum)
        print(line)

if __name__ == "__main__":
    main()
