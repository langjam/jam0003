require_relative "lexer.rb"
require_relative "parser.rb"
require_relative "transpiler.rb"

file_in_name = ARGV.shift
file_out_name = ARGV.shift

# p file_in_name
# p file_out_name
# p ARGV

code = File.read file_in_name

# p code



tokens = Lexer.new(code).lex
# puts p," -- tokens --",p,"nvm"
# p tokens
tree = Parser.new(tokens).parse
# puts p," -- tree --",p
# p tree
transpiled = Transpiler.new(tree).transpile

transpiled = File.read("src/runtime.fixer.thing.rb") + "\n" + transpiled
# add bclib to file
# transpiled = File.read("src/bclib.rb") + "\n" + transpiled

File.write(file_out_name, transpiled)

# puts "🧀 END OF BS -- START OF 🧀!\n"
puts `ruby #{file_out_name} #{ARGV * " "}`