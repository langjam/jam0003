class Lexer
    
    attr :tokens

    def initialize(source, debug = false)
        @source = source
        @debug = debug
    end

    def lex()

        tokens = []
        code = @source.dup
        p code if @debug
        brackets = []
        next_can_be_identifier = true
        last_was_whitespace = false   # to be able to separate a .b / a(.b) and a&b / a &b 
        loop do
            # p code
            $_ = code
            
            if ~/\A#[^\n]*$/ # single line-comment
                tokens << ["comment_single", $&]
                code = $'
            elsif ~/\A^=begin.+^=end/m
                tokens << ["comment_multi", $&]
                code = $'
            elsif ~/\A([1-9][0-9]*|0)/ # int
                tokens << ["int", $&]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A([1-9][0-9]*|0)\.[0-9]+/ # float
                tokens << ["float", $&]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A'([^']*)'/ # raw string, single quote
                tokens << ["raw_string", $1]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A"((\\.|(?!#\{)[^\\"])*?)"/ # normal string, double quote "abc"
                tokens << ["string", $1]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A"((\\.|[^\\"])*?)#\{/ # "abc#{
                tokens << ["string_interpolate_start", $1]
                code = $'
                brackets << "string"
                next_can_be_identifier = true
            elsif ~/\A\}((\\.|[^\\"])*?)#\{/ && brackets[-1] == "string" # }abc#{
                tokens << ["string_interpolate_middle", $1]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\}((\\.|[^\\"])*?)"/ && brackets[-1] == "string" # }abc"
                tokens << ["string_interpolate_end", $1]
                code = $'
                brackets.pop
                next_can_be_identifier = false
            elsif ~/\A`((\\.|[^\\`])*)`/ # shell string, backtick
                tokens << ["shell", $1]
                code = $'
            elsif ~/\A&((?!\d)\w+)/ && (last_was_whitespace || next_can_be_identifier) # &puts
                tokens << ["amp_identifier", $1]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A\.((?!\d)\w+)/ && next_can_be_identifier #.to_a
                tokens << ["dot_identifier", $1]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A\?([^ ])/m && next_can_be_identifier # ?a
                tokens << ["char", $1]
                code = $'
                next_can_be_identifier = false

                #have to separate + and - from infix
                #in `p -a` `p - a`
                # !== === ... <=> -> .. != =~ !~ == || && ** <= >= = ^ | & % / * - + ? : < > ,
                # &&= ||= **= += -= *= /= %=  &= |= ^= << >> <<= >>=
            elsif ~/\A(\+\+|--|~|!(?![=~]))/ || ~/\A(!==|===|&&=|<<=|>>=|\|\|=|\*\*=|\.\.\.|\.\.|<=>|<<|>>|->|\+=|-=|\*=|-=|\/=|%=|&=|\|=|\^=|<=|>=|!=|=~|!~|==|\|\||&&|\*\*|<|>|=|\^|\||&|%|\/|\*|-|\+|\?|:|,)/ # operators
                tokens << ["operator", $&]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\$./ #  $<
                tokens << ["special_dollar", $&]
                code = $'
            elsif ~/\A\$\w+/ # $asd
                tokens << ["dollar", $&]
                code = $'
            elsif ~/\A(do|while|end|until|loop|def|end|lambda|if|else|case|when|elsif|in)(?!\w)/
                tokens << ["keyword", $&]
                code = $'
            elsif ~/\A(true|false)(?!\w)/
                tokens << ["bool", $&]
                code = $'
            elsif ~/\A(?!\d)\w+/ # asd
                tokens << ["identifier", $&]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A\./ # .
                tokens << ["dot", "."]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\r?\n/ # newline
                tokens << ["newline", $&]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\s+/ # whitespace
                tokens << ["whitespace", $&]
                code = $'
            elsif ~/\A;/ # semicolon
                tokens << ["semicolon", ";"]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\{/
                tokens << ["left_curly", "{"]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\}/
                tokens << ["right_curly", "}"]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A\[/
                tokens << ["left_square", "["]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\]/
                tokens << ["right_square", "]"]
                code = $'
                next_can_be_identifier = false
            elsif ~/\A\(/
                tokens << ["left_paren", "("]
                code = $'
                next_can_be_identifier = true
            elsif ~/\A\)/
                tokens << ["right_paren", ")"]
                code = $'
                next_can_be_identifier = false
            else
                STDERR.puts "couldn't parse #{code}"
                exit 1
            end
            p tokens[-1] if @debug
            last_was_whitespace = tokens[-1][0] == "whitespace"
            if code == ""
                return @tokens = tokens.map{|k,v|
                    {
                        "type" => k, "value" => v
                    }
                }
            end


        end

    end
    def to_s(sep = "")
        @tokens.map{|type, value|
            case type
            when "int"; value
            when "float"; value
            when "operator"; value
            when "identifier"; value
            when "raw_string"; "'#{value}'"
            when "string"; "\"#{value}\""
            when "string_interpolate_start"; "\"#{value}\#{"
            when "string_interpolate_middle"; "}#{value}\#{"
            when "string_interpolate_end"; "}#{value}\""
            when "shell"; "`#{value}`"
            when "amp_identifier"; "&" + value
            when "dot_identifier"; "." + value
            when "special_dollar"; value
            when "dollar"; value
            when "keyword"; value
            when "dot"; value
            when "left_curly"; value
            when "right_curly"; value
            when "left_square"; value
            when "right_square"; value
            when "left_paren"; value
            when "right_paren"; value
            when "whitespace"; value
            when "comment_single"; value
            when "comment_multi"; value
            when "semicolon"; value
            when "char"; "?#{value}"
            when "bool"; value
            else; "'''other #{value}'''"
            end

        }.map{|x|
            block_given? ? (yield x) : x
        }.join(sep)
    end
end