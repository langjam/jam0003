# base object
class Bobject
    def initialize(*args, &block)
        @args = args
        @block = block
    end
    def then(func)
        func.call self
    end
end
# enumerator
class Benumerator < Bobject
    def initialize(enum)
        @enum = enum
    end
    def map(func)
        res = Barray.new []
        self.each(lambda{|x|res << func.call(x)})
        res
    end
    def each(func = nil)
        func ? @enum.each(&func) : self # ruby each
    end
    def inspect()
        @enum.inspect
    end
end
# hash
class Bash < Benumerator
    def initialize(hash)
        @hash = hash
    end
    def inspect()
        @hash.inspect
    end
end
# range
class Brange < Benumerator
    def initialize(start, eend, is_inclusive)
        @start = start
        @end = eend
        @is_inclusive = is_inclusive 
    end
    def inspect()
        "(#{@start}#{@is_inclusive ? ".." : "..."}#{@end})"
    end
    def self.inclusive(start, eend)
        Brange.new(start, eend, true)
    end
    def self.exclusive(start, eend)
        Brange.new(start, eend, false)
    end
    def each(func)
        val = @start
        while (val <=> @end) == -1 do
            func.call val
            val = val.succ
            if (val <=> @end) == 0
                func.call val if @is_inclusive
                break
            end
        end
        self
    end
end
# array
class Barray < Benumerator
    def initialize(arr)
        @arr = arr
    end
    def map(block)
        Barray.new @arr.map(&block)
    end
    def each(block)
        @arr.each(&block)
    end
    def inspect
        @arr.inspect
    end
    def push(obj)
        @arr.push obj
    end
    alias << push
end
# string
class Bring < Bobject
    def initialize(str)
        @str = str
    end
    def inspect()
        @str.inspect
    end
end
# int
class Bint < Bobject
    attr :num
    def initialize(num)
        @num = num
    end
    def inspect()
        @num.inspect
    end
    alias to_s inspect
    def succ()
        Bint.new @num + 1
    end
    def <=>(other)
        raise TypeError unless self.class === other
        return @num <=> other.num
    end
end
# float
class Bloat < Bobject
    def initialize(num)
        @num = num
    end
    def inspect() @num.inspect end
end
# function
class Bunction < Bobject
    def initialize(func = nil, &block)
        @func = func || block
    end
    def call(*args)
        @func.call *args
    end
    def inspect()
        @func.inspect
    end
end
# true
class Brueclass < Bobject
    def initialize()
    end
    def to_s() Bring.new("true") end
    alias inspect to_s
    def to_i() Binteger.new(1) end
    def to_f() Bloat.new(1.0) end
end
# false
class Balseclass < Bobject
    def initialize()
    end
    def to_s() Bring.new("false") end
    alias inspect to_s
    def to_i() Binteger.new(0) end
    def to_f() Bloat.new(0.0) end
end
# nil
class Bilclass < Bobject
    def initialize()
    end
    def to_s()
        Bring.new("nil")
    end
    alias inspect to_s
    def to_i()
        Binteger.new(0)
    end
    def to_f()
        Bloat.new(0.0)
    end
end
# overwrite existing methods and classes

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


## end of bclib