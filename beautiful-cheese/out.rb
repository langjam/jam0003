module Enumerable
    alias _old_map map; def map(block = nil, &ob) = ob ? _old_map(&ob) : block ? _old_map(&block) : _old_map # &op keep compatibility with interals that use #map, for example Enumerator#with_index
    
end
class Enumerator
    alias _old_each each; def each(block = nil, &ob) = ob ? _old_each(&ob) : block ? _old_each(&block) : _old_each # &op keep compatibility with interals that use #map, for example Enumerator#with_index
    alias _old_with_index with_index; def with_index(block = nil); block ? _old_with_index(&block) : _old_with_index end
end
class Array
    alias all all?
    alias any any?
    alias one one?
    alias none none?
    alias has include?
    
    # replace "all" methods so they take a lambda, not block
    alias _old_max max; def max(block = nil) = block ? map(&block)._old_max : _old_max
    alias _old_min min; def min(block = nil) = block ? map(&block)._old_min : _old_min
    alias _old_map map; def map(block = nil, &ob) = ob ? _old_map(&ob) : block ? _old_map(&block) : _old_map # &op keep compatibility with interals that use #map, for example Enumerator#with_index
    alias _old_each each; def each(block = nil) = _old_each(&block) 
    alias _old_all all?; def all(block = ->x{x}) = map(block).all?(&block)
end
class Integer
    alias _old_times times
    def times(block = nil) 
        (0..self).each(&block)
        # block ? _old_times(&block) : _old_times
        _old_times(&block)
    end
end
class Object
    def then(block)
        block.call(self)
    end
end
def lambify(name)
    eval "alias _old_#{name} #{name};
    undef :#{name};
    lambda{|*x|_old_#{name}(*x)}", TOPLEVEL_BINDING
end
# https://ruby-doc.org/core-3.1.2/Kernel.html
puts = lambify("puts")
putc = lambify("putc")
print = lambify("print")
printf = lambify("printf")
warn = lambify("warn")
p = lambify("p")
rand = lambify("rand")
# eval = lambify("eval")  #eval is special case, I imagine we want to eval as bc, not as rb? also this breaks lambify

#next todo? &asfs ?
# ok
# hm
# now puts doesn't "implicitly" call, you need .call or .()
# this code runs, try it!

## end of runtime.fixer.thing.rb
a = [1, 2, 3]
a.each(puts)
p.call(a.map(lambda{|x| x.to_s(2) }))
5.times(lambda{|*var_04195502892990014427| puts.call("🧀") })