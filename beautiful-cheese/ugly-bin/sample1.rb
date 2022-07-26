a,b=*$<
a=a.upcase.scan(/[A-Z]/).tally.map{_1+b*_2}.sort
(a.map(&:size).max-1).downto(0){|i|puts a.map{_1[i]||' '}.join.rstrip}

=begin
a, b= ⏩
a=a.upcase.scan(/[A-Z]/).tally.map((x,y)→x+b*y).sort
(a.map(.size).max-1).downto(0, i → puts a.map(a → a[i] ∨ ' ' ).join.rstrip)







h = h.transform_keys .to_s
p h
s = h.keys.sort
p (0..).find(i → p h.keys.reverse.sort_by(k → k[0,i]) == s)
p h.transform_keys(k → k[0,7])
p h.sort_by{|k,v|k.sum+k[3].sum}.to_h.values
a = h.keys.map{|x|x.sum+x[3].sum}
p a,""
sol = []
min = 1000
f  = -> a,l,c{
	poss = (1...l).select{|i|!a.map{|n|n % i}.uniq!}
	if poss.size == 0
        # p c if rand < 0.01
		if l < min 
			p c
			min = l
			sol << c
		end
	else
		poss.each{|x|f[a.map{|v|v % x},x,[*c,x]] if l - 1 != x}
	end
}
f[a,1000,[]]
p sol

=end