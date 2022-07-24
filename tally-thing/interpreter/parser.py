from pyparsing import Word, alphas

greet = Word(alphas) + "," + Word(alphas) + "!"

def parse_file(file):
  return greet.parse_file(file)