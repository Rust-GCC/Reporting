#!/usr/bin/env python3
import optparse, sys, os, re, json, ntpath

NO_VALUE = None
DEFAULT_OUTPUT_DIRECTORY = os.getcwd ()
TEST_CASE_TITLES_LOWERCASE = ["passing", "failed", "xfail", "xpass"]
BLANK_INPUTS = ["-", "", None]

def parse (file):
    foundTestCaseTable = False
    testCase = {}

    print ("parsing file %s" % file)
    print ("File Table:")
    with open (file, "r") as f:
        for line in f:
            line = line.rstrip ()
            if line == None or line == "" or line == " ":
                continue
            if line == "*** Test Cases":
                foundTestCaseTable = True
                continue
            if not foundTestCaseTable:
                continue
            if line.startswith ("***"):
                break

            print (line)
            line = line.replace (" ", "").lower ()
            row = line.split ("|")
            if len (row) != 6:
                continue

            _,title,previous,current,delta,_ = row
            if title not in TEST_CASE_TITLES_LOWERCASE:
                continue

            testCase[title] = {}
            hasPrevious =  previous not in BLANK_INPUTS
            hasCurrent = current not in BLANK_INPUTS

            if hasPrevious:
                testCase[title]["previous"] = int (previous)
            else:
                testCase[title]["previous"] = NO_VALUE
            if hasCurrent:
                testCase[title]["current"] = int (current)
            else:
                testCase[title]["current"] = NO_VALUE

            testCase[title]["delta"] = NO_VALUE
            if hasPrevious and hasCurrent:
                testCase[title]["delta"] = int (current) - int (previous)
            if not hasPrevious and hasCurrent:
                testCase[title]["delta"] = int (current)
            if hasPrevious and not hasCurrent:
                testCase[title]["delta"] = NO_VALUE

    return testCase


def save_json (fullFilePath, dictionary):
    with open (fullFilePath, "w") as f:
        json.dump (dictionary, f)
    print ("json output: %s" % fullFilePath)


def singleFileParse (option):
    testCase = parse (file=option.file)
    print (testCase)
    filename = ntpath.basename (option.file)
    filename = filename.replace ("report.org", "test-case.json")

    fullFilePath = os.path.join (option.output, filename)

    save_json (fullFilePath=fullFilePath, dictionary=testCase)
    sys.exit (os.EX_OK)


def allFileParse (option):
    allFilesParsed = {}
    for year in os.listdir ("."):
        if not re.search (r'^[0-9]{4}$', year):
            continue

        for file in os.listdir (os.path.join (".", year)):
            if not re.search (r'^[0-9]{4}-[0-9]{2}-[0-9]{2}-report.org', file):
                continue
            name = file.replace ("-report.org", "")
            fullFilePath = os.path.join (os.getcwd (), year, file)
            allFilesParsed[name] = parse (file=fullFilePath)

    fullFilePath = os.path.join (option.output, "all-test-case.json")
    save_json (fullFilePath=fullFilePath, dictionary=allFilesParsed)
    sys.exit (os.EX_OK)


if __name__ == "__main__":
    parser = optparse.OptionParser ()

    parser.add_option ("-d", "--date",
                       type = "string",
                       dest = "date",
                       default = None,
                       help = ("Select date to parse, format: 2024-09-30 "
                               "default: latest"))

    parser.add_option ("-f", "--file",
                       type = "string",
                       dest = "file",
                       default = None,
                       help = ("Parse target file (absolute path) rather than "
                               "date, default: None"))

    parser.add_option ("-o", "--output",
                       type = "string",
                       dest = "output",
                       default = DEFAULT_OUTPUT_DIRECTORY,
                       help = ("Output directory, Default: %s" %DEFAULT_OUTPUT_DIRECTORY))

    parser.add_option ("-a", "--all",
                       action = "store_true",
                       dest = "parseAll",
                       default = False,
                       help = "Parse all files, Toggle")

    option, arg = parser.parse_args ()

    if option.parseAll:
        allFileParse (option)

    if option.file:
        singleFileParse (option)

    if not option.date:
        direc = max ([
            f for f in os.listdir (".")
            if re.search (r'^[0-9]{4}$', f)
        ])
        fileName = max ([
            f for f in os.listdir (os.path.join (".", direc))
            if re.search (r'^[0-9]{4}-[0-9]{2}-[0-9]{2}-report.org',
                          f)
        ])
        option.file = os.path.join (os.getcwd (), direc, fileName)

        singleFileParse (option)

    direc = option.date[0:4]
    fileName = option.date+"-report.org"
    option.file = os.path.join (os.getcwd (), direc, fileName)

    singleFileParse (option)
