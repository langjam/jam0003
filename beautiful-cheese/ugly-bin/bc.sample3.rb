# double = |x| x * 2
# triple = | a |a * 3
# add = |x, y| x + y

# p double 10
# puts triple 123
# p add 1,2

# dosomething = || puts rand 10
# f = dosomething
# p f
# puts "."
# p 5.times.map(|x|x)
# id=|x|x
# prog = |x|(p 0; p 1
# p 3); p 2
# p 5.times &id
# prog 0
# prog 0
# [1,2,3].map(|x|p x + 1)
# f = |x,y|(p x,y);
# # 5.times.map(&f)
# p 5.times(||p p)


# p [1,2,3].then(|x,*y|[x,y])
# r = (1..10)
# p r
# true ? (
#     puts "yes"
#  ):(1)
# puts "ok"

5.times(|arg| puts "hello " + arg.&to_s() + "!")
# puts "ko"
# p 1 * 2 + 3 * 4
# p *(0..999).map(f=|n|n<1?0:1+f(n%2<1?-~n*3:n/2))
# a = [1,2,3]
# p a[2]
# a = 1 + (b = 2)

# p *[a,b], c = 2
# p ++c

# {"type"=>"int", "value"=>"5"},
# {"type"=>"dot", "value"=>"."},
# {"type"=>"identifier", "value"=>"times"},
# {"type"=>"left_paren", "value"=>"("},
# {"type"=>"operator", "value"=>"|"},
# {"type"=>"identifier", "value"=>"arg"},
# {"type"=>"operator", "value"=>"|"},
# {"type"=>"whitespace", "value"=>" "},
# {"type"=>"identifier", "value"=>"puts"},
# {"type"=>"whitespace", "value"=>" "},
# {"type"=>"string", "value"=>"hello "},
# {"type"=>"whitespace", "value"=>" "},
# {"type"=>"operator", "value"=>"+"},
# {"type"=>"whitespace", "value"=>" "},
# {"type"=>"identifier", "value"=>"arg"},
# {"type"=>"dot", "value"=>"."},
# {"type"=>"identifier", "value"=>"to_s"},
# {"type"=>"whitespace", "value"=>" "},
# {"type"=>"operator", "value"=>"+"},
# {"type"=>"whitespace", "value"=>" "},
# {"type"=>"string", "value"=>"!"},
# {"type"=>"right_paren", "value"=>")"},
# {"type"=>"newline", "value"=>"\n"},
# {"type"=>"comment_single", "value"=>"# puts \"ko\""},
# {"type"=>"newline", "value"=>"\n"},
# {"type"=>"comment_single", "value"=>"# p 1 * 2 + 3 * 4"},
# {"type"=>"newline", "value"=>"\n"},
# {"type"=>"comment_single", "value"=>"# p *(0..999).map(f=|n|n<1?0:1+f(n%2<1?-~n*3:n/2))"},
# {"type"=>"newline", "value"=>"\n"}