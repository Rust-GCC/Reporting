#!/usr/bin/env bash

python ./util/pr-test-case-to-json.py -a
python ./util/pr-test-case-plot-all.py -f all-test-case.json

rm ./all-test-case.json
