# worship the big beautiful cheese
# the program will be an assembly of individuals who are extreme caseophile's 
# it will have lots of the unicode
# the program will look cool

# changes to make:
# a.map(.asd(2)) -> a.map{|x|x.asd(2)}
# a.map(&asd) -> a.map{|x|asd x}
# &a -> method(:a)
# a.&b -> a.method(:b)
# .asd -> lambda{|x|x.asd}
# .asd(123) -> lambda{|x|x.asd(123)}
# {|x| code } -> lambda{|x| code } ? maybe

f = { |x| code }
#now:
f = lambda{ |x| code }
f = -> (x) { code }

༼ つ ◕_◕ ༽つ # their name is bob
# hi bob
#
bob 🧍
bob.🛐 # bob worships the cheese
🧀 bob.💪 # the cheese declares bob's status
anita 🧍
anita ⬅️ bob # set anita to copy bob's stats
🧀 anita # cheese declares anita 
☀️ # cheese die


alias 🧀 puts
alias ☀️ die_func
🧀 'hello'
️
-----------------------️

--- testing syntax ---

Glang  = require "./glang.js"
fs = require "fs"

a ⊂ b ⊇ c
(⊢ + ⊣) I love apl syntax and stuff
and pointfree
got more tacks than tackshooter

# ⊃⊄⊅⊆⊇⊈⊉⊊⊋⊌⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊝⊞⊟⊠⊡⊢⊣⊤⊥⊦⊧⊨⊩⊪⊫⊬⊭⊮⊯⊰⊱⊲⊳⊴⊵⊶⊷⊸⊹⊺⊻⊼⊽⊾⊿⋀⋁⋂⋃⋄⋅⋆⋇⋈⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋔⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌀⌁⌂⌃⌄⌅⌆⌇⌈⌉⌊⌋
a.map{|x|f x}
a.map{|x|x.f}
a.map(f)
a.map(x -> x.f)
a.map(x→x.f)
a.map(x↝x.f)
a.map(.to_s(2)) # that is nice, instead of lambda {|x|x.f
that is obj method
that vv is global "func" method
"f(a)" vs "a.f()"
[].map(f) # not possible in ruby yet
[].map(&method(:f)) # how to do currently
[].map(&f) # how to solve if
[].map(.f)
[].map(&(a, b) => a+b) 
[].map((a, b) -> a+b) 
[].map (+1)
[].map(&+((1)))


# [].map(.+1)
# ok
#partial application as syntax idk 
# I know
f((1)) == lambda{|*args| f(1,*args)}
# so if the function only takes 1 item?
# &f is like method(:f), it referenced the function but doesn't call it
#could also do:
a.&to_s
#instead of
a.method(:to_s)