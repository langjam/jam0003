# 
# remember that in &asd   abc.&asd   a.map(&asd)  &asd is a single token
# same in map(.to_i)  map(.to_i(1))  .to_i is a single token
#old ruby?
#can you share terminal?
class ParserBAD
    def initialize(token_arr)
        @token_arr = token_arr.to_a.select{|x| x[0] != "whitespace"} 
        @token_curr = nil
        @token_ind = -1
        @ast_arr = []
        p @token_arr
        eat()
    end

    def eat() 
        @token_ind += 1
        @token_curr = @token_arr[@token_ind]
        @token_curr
    end

    def typ(tok)
        tok[0]
    end

    def val(tok)
        tok[1]
    end

    def peek() 
        # @token_ind += 1
        @token_arr[@token_ind + 1]
    end

    def parse()
        # ast = consume
        # eat()

        curr = consume()
        @ast_arr << curr
        while curr != nil
            curr = consume()
            @ast_arr << curr
        end
        @ast_arr.pop

        @ast_arr
    end

    def factor()
        tok = @token_curr
        eat()
        if tok == nil then
            nil
        elsif typ(tok) == "left_paren"

            fac_res = {"type" => "paren_expr", "value" => consume()}

            if typ(@token_curr) != "right_paren"
                warn "oh no paren error"
                exit 69
            else
                eat()
            end

            fac_res
        else
            {"type" => typ(tok), "value" => val(tok)}
        end
    end

    # def term()

    # end

    def consume()
            tok_typ, tok_val = @token_curr

            left = factor()

            while @token_curr != nil && ["operator"].include?(typ(@token_curr)) do
                op = @token_curr
                eat()
                right = factor()
                # right = @token_curr

                left = {"type" => typ(op), "name" => val(op), "arguments" => [left, right]}
            end
            left
    end
    
end