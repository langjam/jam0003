# run using `pipenv run python3 . examples\test.tally examples\test.json`

import argparse

intake = argparse.ArgumentParser(
  description='Interpreter for the tally-thing DSL'
)
intake.add_argument('file', help='the tally script file to be interpreted')
intake.add_argument('data', help='the data file to be processed by your script')
args = intake.parse_args()

import json

from parser import parse_file
from runner import run_program

program = parse_file(args.file)
with open(args.data, "r") as data_file:
  data = json.load(data_file)
result = run_program(program, data)
print(result)