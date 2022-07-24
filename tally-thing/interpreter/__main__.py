# run using `pipenv run python3 . examples\test.tally`

import argparse

intake = argparse.ArgumentParser(
  description='Interpreter for the tally-thing DSL'
)
intake.add_argument('file', help='the tally script file to be interpreted')
#intake.add_argument('data', help='the data file to be processed by your script')
args = intake.parse_args()

print("You wish to interpret: ", args.file)
print("Unfortunately, I can't. Yet.")

from parser import parse_file

print("\nInstead, I'll at least try and parse it:")
print(parse_file(args.file))