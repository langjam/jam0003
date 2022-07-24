class Program:
  def __init__(self, datas, lets, exprs):
    self.datas = datas
    self.lets = lets
    self.exprs = exprs
  def __repr__(self):
    return (
        "<program\n\n  datas: " + str(self.datas) +
        "\n\n  lets: " + str(self.lets) + 
        "\n\n  exprs:" + " ".join(str(x) for x in self.exprs) + 
      "\n>")

class LetBody:
  def __init__(self, params, exprs):
    self.params = params
    self.exprs = exprs
  def __repr__(self):
    return "(" + ", ".join(self.params) + ") => " + " ".join(str(x) for x in self.exprs) + "\n----"

class Term:
  __match_args__ = ('name', 'params')
  def __init__(self, name, params):
    self.name = name
    self.params = params
  def __repr__(self):
    more = ""
    if self.params:
      more = "(" + ", ".join(self.params) + ")"
    return "<term " + self.name + more + ">"

class BagVal: #not actually a value, but a function bag->bag
  def __init__(self, exprs):
    self.exprs = exprs
  def __repr__(self):
    return "<bag " + " ".join(str(x) for x in self.exprs) + ">"

class RecordVal: #as above
  def __init__(self, dicty):
    self.dicty = dicty
  def __repr__(self):
    return "<record " + str(self.dicty) + ">"

if __name__ == "__main__":
  term = Term("hello", ["x", "y"])
  print(term)