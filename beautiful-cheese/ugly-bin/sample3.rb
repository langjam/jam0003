
f=->s,b{s[1]?b ? ?(+f[s[..-2],!b]+") "+s[-1]:s[0]+" ("+f[s[1..],!b]+?): s[0]}

f=|s,b|(s[1]?b ? ?(+f[s[..-2],!b]+") "+s[-1]:s[0]+" ("+f[s[1..],!b]+?): s[0])
#how to define lambdas?
# dont need to
# ok
#yes you do?
f=(s,b)->(a;b;c)
f=(s,b)->s+b
f={|s,b|s+b}
f=(|s,b|s+b)
f=|s,b|s+b
f=|s,b|(s+b;d;e;f) this?
§1234567890+´½!"#¤%&/()=?`@£$€{[]}\¨^~'*-_.:,;µ<>|"
bar | looks best by far, replace logic or with ∧ ∨  ascii V as or?
?
# to fix v
# to fix v
# what wrong with v is logical or, but I think we can work around that
|| as prefix kinda
then you will always need () when using |adas|adasd but thats fine?
# yeah should be fine
5.times(||puts rand)# isn't rust like this?
(0..5).map(|_|puts rand)# isn't rust like this?
#hat actually works and its "beautiful"
# yayayayayayay
# lol
#idk how to diff tuple vs "math parens"
# doesn't have to be different, now tuple is a thing? sure
# it can be simliar to the delimiited list yes, either tuple -> body or ident -> body
# parse a tuple thing then check maybe_function
#lets think about this
#what if you want 5.times{puts rand}
#5.times(()->puts(rand))?
#5.times{puts rand} do we keep trailing {} blocks, and transpile them to normal lambda 
#t.times{puts rand} -> 5.times(lambda{puts rand}) ?
# idk
#{} blocks is kinda nice ... 
5.times(&rand).map(&puts)
5.times〈 〉⟨⟩
5.times{puts rand} #trying to find something more "beautiful" than {}
# I think we can keep as is {|x| x } maybe {⁰¹} instead of _1 _2
[1,2,3].map{p ⁰①②}
[1,2,3].map「 」{p ⁰}
[1,2,3].map &p
p *[1,2,3]
a = 
「 」
🤔
🤔
$><<f[gets.split,0]

# f=(s,b)→s[1]?b?"("+f(s[..-2],!b)+") "+s[-1]:s[0]+" ("+f(s[1..],!b)+")":s[0]}
# ⏪f(gets.split,0)


# raku? 
# kinda
# a.f(*g) === f(a,*g) ?
# it looks clean, but I think it maybe has gone too far idk, ok
# lol alright it might be confusing
# its no longer changes in syntax, but that shouldnt really matter, but its a big ... thing to implement
# so maybe not then
# lets start code  the first parts then?
# sure
# I will try to tokenize almost all possible ruby code, not as ruby does it, but as it should be done :)
# alright
