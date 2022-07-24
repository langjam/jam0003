LOG = false
# PRECEDENCE = {
#     "="=> 2,
#     "||"=> 4,
#     "?"=> 3,
#     "=>"=> 3,
#     "&&"=> 5,
#     "<"=> 10, ">"=> 10, "<="=> 10, ">="=> 10,
#     "=="=> 9, "!="=> 9,
#     "+"=> 12, "-"=> 12,
#     "*"=> 13, "/"=> 13, "%"=> 13,
#     "++"=> 14, "--"=> 14,
# }
PRECEDENCE = [
    "[] []=",
    "**",
    # "! ~ + -",
    "* / %",
    "+ -",
    ">> <<",
    "&",
    "^ |",
    "<= < > >=",
    "<=> == === != =~ !~",
    "&&",
    "||",
    ".. ...",
    "? :",
    "= %= { /= -= += |= &= >>= <<= *= &&= ||= **=",
    "defined?",
    "not",
    "or and",
    "if unless while until",
    "begin end"
].reverse.flat_map.with_index{|row, i|
    row.split.map{|key|
        [key, i + 1]
    }
}.to_h
# p PRECEDENCE

class Parser
    def initialize(tokens)
        @tokens = tokens.select{|x| x["type"] != "comment_single"}
    end

    def parse
        parsed = []
        while @tokens.size > 0
            # p "trying to parse next expr:"
            expression = parseExpression()
            # p "expression:"
            # p expression
            parsed << expression
        end
        return {"type" => "program", "program" => parsed}
    end
    def next_token()
        @tokens[0] && @tokens.shift()
    end

    def parseExpression(as_arg = false) 
        expression = parseAtom(as_arg)
        # expression = parse_args_no_paren(expression)
        p ["expr parseatom", expression] if LOG
        expression = maybe_square(expression)
        expression = maybe_call(expression, call_need_parens(expression))
        p ["expr maybecall", expression, call_need_parens(expression)] if LOG
        expression = maybe_method(expression)
        p ["expr maybemethod", expression] if LOG
        expression = maybe_binary(expression, 0)
        p ["expr binary", expression] if LOG
        p "expr: #{expression}" if LOG
        return expression
        # call is making them all nil
    end
    def parseParenExpression()
        # puts "parenexpr"
        next_token()
        expressions = []
        loop do
            # p 0
            # p @tokens[0]
            if isNextType("right_paren")
                next_token()
                return {
                    "type" => "expressions",
                    "value" => expressions
                }
            end
            expression = parseExpression()
            expressions << expression
            # p @tokens[0]
            skipAllWhiteSpace()
            p @tokens[0]
        end

    end
    def parseAtom(as_arg = false)
        return {"type"=> "nil"} if @tokens.size == 0 
        
        first = @tokens[0]
        type = first["type"]
        value = first["value"]
        puts "first: #{first}" if LOG
        if type == "left_paren"
            return parseParenExpression()
        end
        
        if type =~ /(int|float|string|bool|nil)/
            next_token()
            return {
                "type" => "literal",
                "value" => first
            }
        end
        if type == "dot_identifier"
            skipNextType("dot_identifier")
            args = parse_args(first)["args"]
            return {
                "type" => "dot_identifier",
                "name" => value,
                "args" => args,
                "as_arg" => as_arg
            }
        end
        if type == "identifier"
            skipNextType("identifier")
            return first
        end
        if type == "whitespace"
            next_token()
            return parseAtom()
        end
        if value == "[" 
            puts "delimited [ ] ," if LOG
            values = delimited("[", "]", [","], method(:parseExpression))
            return {
                "type" => "array",
                "value" => values
            }
        end
        if type =~ /comment/
            next_token()
            return parseAtom()
        end
        if type == "newline" || type == "semicolon"
            next_token()
            return parseAtom()
        end 
        # if type == "right_paren"
        #     next_token()
        #     return NOTHING
        # end 
        if type == "operator"

            # check for (|args|body) function
            puts "in prefix" if LOG
            p value if LOG
            if value == "|" || value == "||"
                return parse_func()
            end

            # else prefix operator
            puts "prefix" if LOG
            next_token()
            atom = parseAtom()
            return value == "*" ? {
                "type" => "splat",
                "value" => atom
            } : {
                "type" => "prefix",
                "operator" => value,
                "right" => atom
            }
        end
        if type == "amp_identifier"
            next_token()
            return first
        end
        if type == "special_dollar"
            next_token()
            return first
        end
        # hacky, forward it to ruby later
        # return {
        #     "type" => "raw",
        #     "value" => first
        # }
        puts("didn't parse:") if LOG
        puts(first) if LOG
        throw "Didn't parse an atom"
    
    end
    def maybe_square(expression)
        if isNextType("left_square")
            args = delimited("[", "]", ",", method(:parseExpression))
            return {
                "type" => "index",
                "self" => expression,
                "args" => args
            }
        end
        expression
    end
    def skipNextType(type)
        if @tokens[0]&.[]("type") == type
            return @tokens[0] && @tokens.shift()
        end
        throw "Invalid token to skip"
    end
    
    def skipNextValue(value)
        if @tokens[0]&.[]("value") == value
            return @tokens[0] && @tokens.shift()
        end
        puts "can't skip #{value}, next value is #{@tokens[0]&.[]("value")}" if LOG
        throw "Invalid value to skip"
    end

    def isNextType(type)
        return @tokens[0]&.[]("type") == type
    end

    def isNextValue(value)
        return @tokens[0]&.[]("value") == value
    end
    
    def delimited(start, eend, (separator_value, separator_type), parser_func)
        parsed = []
        first_iteration = true
        skipNextValue(start) if start
        
        while @tokens.length > 0
            skipAllWhiteSpace()
            break if String === eend ? isNextValue(eend) : @tokens[0]["value"] =~ eend #this should stop v
            
            if(first_iteration)
                first_iteration = false
            else
                # it trying to delimit the comment? idk
                # now it says it collected /demilited "a", but then keeps looking, doesn't see newline
                #  p ["is next val", separator_value, isNextValue(separator_value)]
                # p ["real next val", @tokens[0]]
                skipAllWhiteSpace()
                if(separator_value && isNextValue(separator_value)) # where is this false  #this from being false ri ght?
                    # nil is false i think if it's false that won't run
                    # nil is just false in ruby
                    puts("skipping next '#{separator_value}'") if LOG
                    skipNextValue(separator_value)
                else
                    LOG && puts("skipping next '#{separator_type}'")
                    puts(separator_value) if LOG
                    skipNextType(separator_type) # skip nil?
                end
            end
            
            # I think isNextValue() here it is exact, but the token is any whitespace
            # break if Array === eend ? eend.any?{|x| isNextValue(x)} : isNextValue(eend) #check here no need to check down there
            break if String === eend ? isNextValue(eend) : @tokens[0]["value"] =~ eend #check here no need to check down there
            #  method(:parseExpression) f[] === f.() === f.call()
            expression = parser_func.()
            # break unless expression
            parsed.push(expression)
        
        end
        # think it's that nil you passed earlier to delimited no, that was start
        
    
        # skipNextValue(eend)
        next_token()
        p @tokens[0] if LOG
        "return from delimited"
        return parsed
    end
    
    def skipAllSpace()
        skipped = false
        (skipped = true; next_token()) while isNextType("whitespace")
        skipped
    end
    def skipAllWhiteSpace()
        skipped = false
        (skipped = true; next_token()) while isNextType("whitespace") || isNextType("newline")
        skipped
    end
    def skipAllHoriSpace()
        skipped = false
        (skipped = true; skipNextType("whitespace")) while isNextType("whitespace")
        skipped
    end

    def maybe_binary(left, precedence)
        # ignore space
        skipNextType("whitespace") while isNextType("whitespace")
        is_operator = isNextType("operator") && !isNextValue(",") #master bug remover large brain
        operator = @tokens[0]

        # p ["in maybe binary is operator?", is_operator]
        if(is_operator)
            p operator["value"] if LOG
            other_precedence = PRECEDENCE[operator["value"]]
            # comma isn't operator right? it's syntax
            # you didn't pass in the type for the separator
            # maybe that mess it up
            if(other_precedence > precedence)
                if(operator["value"] == "?")
                    condition = left
                    skipNextValue("?")
                    truthy = parseExpression()
                    skipNextValue(":")
                    falsy = parseExpression()
                    return {
                        "type" => "ternary",
                        "condition" => condition,
                        "truthy" => truthy,
                        "falsy" => falsy
                    }

                end
                return left if operator["value"] == ":"

                skipNextType("operator")
                atom = parseAtom()
                right = maybe_binary(atom, other_precedence)
                need_parens = operator["value"] !~ /^=/
                right = maybe_call(right, need_parens)
                right = maybe_method(right)
                binary = {
                    "type" => "binary_operation",
                    "operator" => operator["value"],
                    "left" => left,
                    "right" => right
                }
                binary["type"] = "assignment" if operator["value"] == "="
                return maybe_binary(binary, precedence)
            end
        end

        return left
    end
    
    def isLiteral(expr)
        case expr
        when "int", "float", /string/; true
        else; false
        end 
    end
    def is_prefix_op(op)
        %w"+ - ++ -- ~ ! *".include?(op)
    end
    def is_prefix_only_op(op)
        %w"~ !".include?(op)
    end
    def maybe_call(expression, parens_needed = true)
        p ["in maybe_call", expression, parens_needed] if LOG
        
        if isNextType("left_paren") 
            ret = parse_call(expression)
            p "ret from maybe_call with parens: #{ret}" if LOG
            return ret
        elsif !parens_needed
            skipped_space = skipAllHoriSpace()
            # p "@tokens:"
            # p @tokens
            return expression if @tokens.empty?
            non_whitespace = @tokens.drop_while{|token| token["type"] == "whitespace"}
            # if expression["type"] == "identifier" && !%w"operator dot newline semicolin right_square".include?(@tokens[0]["type"])
            if expression["type"] == "identifier" && !%w"operator dot newline semicolon right_square right_paren".include?(non_whitespace[0]["type"])
                puts "prefix arg no paren 1" if LOG
                return parse_args_no_paren(expression)
            # elsif expression["type"] == "identifier" && skipped_space && is_prefix_op(non_whitespace[0]["value"]) && non_whitespace[1]["type"] !~ /(whitespace|newline)/
            elsif expression["type"] == "identifier" && (is_prefix_only_op(non_whitespace[0]["value"]) || (skipped_space && is_prefix_op(non_whitespace[0]["value"]))) && non_whitespace[1]["type"] !~ /(whitespace|newline)/
                puts "prefix arg no paren 2" if LOG
                return parse_args_no_paren(expression)
            else
                return expression
            end
                did_skip_newline = false
            (did_skip_newline ||= @tokens[0]["value"] =~ /\n/; skipNextType("whitespace")) while isNextType("whitespace")
            
            # p ["in maybe call check token 0", @tokens[0]]
            # expression is kinda @tokens[-1] (previous head)
            if expression["type"] == "identifier" && !did_skip_newline && (isNextType("identifier") || isNextType("literal")) # what?? next type is dot wait
                puts "function without parens named: #{expression["value"]}"
                # aaaaaaa
                # I think it works now, new error
                # check for nil?
                # puts "monoid in a category of endofunctors #"
                # name = next_token() # 
                # p name
                # p @tokens[0,2]
                
                # but not delimited this time
                # hm is the end symbol always [\n;] ?
                # then we have to replace ; with \n or not
                # we can add manually
                #not var, Class Class === var ) == ( var instanceof class)
                # o ok
                # you can do
                # case "ASD" when String # because case uses ===

                args = delimited(nil, /[\n;]/, [","], method(:parseExpression))
                ret = {
                    "type" => "call",
                    "func" => expression,
                    "args" => args
                }
                p "ret from maybe_call no parens: #{ret}" if LOG
                return ret
            end       
        end
        p "ret from maybe_call no call: #{expression}" if LOG
        return expression

    end
    def call_need_parens(expr)
        puts "need parens?" if LOG
        p expr if LOG
        res = case expr["type"]
            when "identifier"; false
            else; true
        end
        p res if LOG
        return res
    end
    
    def parse_call(func)
        # args = delimited("(", ")", [","], method(:parseExpression))
        args = delimited("(", ")", [","], lambda{parseExpression(true)})
        return {
            "type" => "call",
            "func" => func,
            "args" => args,
        }
    end
    def maybe_method(expression)
        # p @tokens
        return isNextType("dot") ? maybe_method(parse_method(expression)) : expression
    end
    def parse_method(func)
        skipNextType("dot")
        throw "method name is not a valid name lol" unless isNextType("identifier") || isNextType("amp_identifier")
        if isNextType("identifier")
            name = next_token()
            args = []
            if isNextType("left_paren") 
                args = parse_args({
                    "type" => "method",
                    "self" => func,
                    "name" => name
                })["args"]
            elsif skipAllSpace() && isNextType("operator") && @tokens[1]["type"] != "whitespace"
                args = parse_args_no_paren({
                    "type" => "method",
                    "self" => func,
                    "name" => name
                })["args"]
            end
            return {
                "type" => "method",
                "self" => func,
                "name" => name["value"],
                "parens" => isNextType("left_paren"),
                "args" => args
            }
        else
            name = next_token()
            return {
                "type" => "amp_method",
                "self" => func,
                "name" => name["value"],
                "parens" => isNextType("left_paren"),
                "args" => isNextType("left_paren") ? parse_args({
                    "type" => "method",
                    "self" => func,
                    "name" => name
                })["args"] : nil
            }    
        end
    end
    def parse_args(func)
        if isNextType("left_paren")
            return parse_call(func)
        else
            return {
                "type" => "call",
                "func" => func,
                "args" => []
            }
        end
    end
    def parse_args_no_paren(func)
        puts "parsing name #{func} args, no parens" if LOG
        args = []
        loop do
            skipAllSpace()
            if isNextType("newline") || isNextValue(";") || isNextType("dot") || isNextValue(")")
                return {
                    "type" => "call",
                    "func" => func,
                    "args" => args
                }
            end
            if isNextType("whiteSpace")
                next_token()
                next
            end
            expr = parseExpression(true)
            args << expr
            puts "parsed no-paren arg: #{expr}" if LOG
            skipAllHoriSpace()
            if isNextValue(",")
                next_token()
            else
                return {
                    "type" => "call",
                    "func" => func,
                    "args" => args
                }
            end

        end

        # delimited(nil, /[\n;]/, [","], method(:parseExpression))
        
    end
    def parse_func()
        args = []
        if @tokens[0]["value"] == "||"
            args = []
            next_token()
        else
            args = parse_func_args()
        end
        puts "args:" if LOG
        p args if LOG
        skipAllWhiteSpace()
        if isNextType("semicolon")
            return {
            "type" => "func",
            "args" => args,
            "body" => {"type" => "nil"}
        }
        end
        body = parseExpression()
        return {
            "type" => "func",
            "args" => args,
            "body" => body
        }
    end
    def parse_func_args
        next_token()
        skipAllHoriSpace()
        args = []
        loop do
            arg = nil
            if isNextType("identifier") 
                arg = @tokens[0]
                next_token()
            elsif isNextValue("*") # splat-argument
                next_token()
                skipAllHoriSpace()
                arg = {
                    "type" => "splat",
                    "value" => @tokens[0]
                }
                next_token()
            end
            args << arg
            skipAllHoriSpace()
            if isNextValue("|")
                next_token()
                return args
            end
            # p @tokens
            skipAllHoriSpace()
            skipNextValue(",")
            skipAllHoriSpace()
        end
    end
end