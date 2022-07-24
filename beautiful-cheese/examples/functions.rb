a = [1, 2, 3]

a.each(&puts)

p a.map(|x|x.to_s(2))

5.times(||
    puts "🧀"
)