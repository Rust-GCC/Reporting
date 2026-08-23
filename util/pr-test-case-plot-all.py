#!/usr/bin/env python3
import os, optparse, sys, json, datetime, ntpath
import matplotlib.pyplot as plt
import matplotlib.dates as mdates


DEFAULT_OUTPUT_DIRECTORY = os.getcwd ()


def main (option):
    with open (option.file, "r") as f:
        allTestCase = json.load (f)

    fig, axs = plt.subplots (2,
                             1,
                             sharex=True,
                             figsize=(10,7),
                             gridspec_kw={'height_ratios': [3,1]})
    fig.subplots_adjust (hspace=0.05)

    yearPlot = None
    dates = []
    passingCurrent = []
    passingDelta = []
    failedDelta = []
    totalTests = []
    for date in sorted (allTestCase):
        dates.append (datetime.datetime.strptime (date,"%Y-%m-%d"))
        try:
            dataPoint = allTestCase[date]["passing"]["current"]
        except:
            dataPoint = 0
        passingCurrent.append (dataPoint)

        try:
            dataPoint = allTestCase[date]["passing"]["delta"]
            if not dataPoint:
                dataPoint = 0
        except:
            dataPoint = 0
        passingDelta.append (dataPoint)

        try:
            dataPoint = allTestCase[date]["failed"]["delta"]
            if not dataPoint:
                dataPoint = 0
        except:
            dataPoint = 0
        failedDelta.append (dataPoint)

        try:
            dataPoint = allTestCase[date]["failed"]["current"]
            if not dataPoint:
                dataPoint = 0
        except:
            dataPoint = 0

        totalTests.append (dataPoint + passingCurrent[-1])


    axs[0].plot (dates, passingCurrent, linewidth=2, label="Passed test cases")
    axs[0].plot (dates, totalTests, linewidth=1, label="Total test cases")
    axs[1].plot (dates, passingDelta, linewidth=2, label="Passed (Δ)")
    axs[1].plot (dates, failedDelta, linewidth=1, label="Failed (Δ)")

    for ax in axs:
        ax.xaxis.set_major_locator (mdates.MonthLocator (bymonth=(1, 7)))
        ax.xaxis.set_minor_locator (mdates.MonthLocator ())
        ax.grid (True)

    axs[0].set (title="Rust GCC Test Cases")
    axs[0].set (ylabel="Test Cases")
    axs[0].legend ()
    axs[1].set (xlabel="Date", ylabel="Deltas (Δ)")
    axs[1].legend ()
    for label in axs[1].get_xticklabels(which='major'):
        label.set(rotation=30, horizontalalignment='right')

    print ("saving image:", option.output)
    plt.savefig (option.output)


if __name__ == "__main__":
    cwd = os.getcwd ()
    parser = optparse.OptionParser ()

    parser.add_option ("-f", "--file",
                       type = "string",
                       dest = "file",
                       default = None,
                       help = "all-test-case.json to graph can't be empty")

    parser.add_option ("-o", "--output",
                       type = "string",
                       dest = "output",
                       default = DEFAULT_OUTPUT_DIRECTORY,
                       help = ("Output directory, Default: %s" %DEFAULT_OUTPUT_DIRECTORY))

    option, arg = parser.parse_args ()

    if not option.file or not os.path.isfile (option.file):
        parser.print_help ()
        sys.exit (1)

    fileName = ntpath.basename (option.file).replace (".json", ".png")
    option.output = os.path.join (option.output, fileName)

    main (option)
