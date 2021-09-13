#!/usr/bin/env python3

import sys


def generate_url(prnum):
    return "https://github.com/Rust-GCC/gccrs/pull/{0}".format(prnum)


def generate_desc(prurl):
    return "FIXME"


def generate_org_line(desc, prurl, prnum):
    return "- {0} [[{1}][PR{2}]]".format(desc, prurl, prnum)


def main():
    for prnum in sys.argv[1:]:
        prurl = generate_url(prnum)
        desc = generate_desc(prurl)
        line = generate_org_line(desc, prurl, prnum)
        print(line)

        
if __name__ == "__main__":
    main()
