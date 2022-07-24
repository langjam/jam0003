c,r=`sed 9d`.split.map(.to_i)
c.times{puts (0...r).map{"%#{(c*r-1).to_s.size+1}d"%~-$.+=1}*""}