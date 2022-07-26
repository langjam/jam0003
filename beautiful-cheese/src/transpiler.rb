class Transpiler
    def initialize(tree)
        @tree = tree
        @use_bclib = false
    end

    def transpile
        f(@tree)
    end

    def f(node)
        type = node["type"]
        value = node["value"]
        case type
        when "program"
            node["program"].map{|expr| f(expr) }.join("\n")
        when "identifier"
            value
        when "method"
            "%s.%s(%s)" % [
                f(node["self"]),
                node["name"],
                node["args"].map{|x| f(x) }.join(", ")
            ]
        when "dot_identifier"
            method_name = node["name"]
            arguments = node["args"].map{|x| f(x) }
            self_name = rand_name()
            res = "lambda{ | %s | %s.%s(%s) }" % [self_name, self_name, method_name, arguments.join(", ")]
            @use_bclib ? "Bunction.new(#{res})" : res
        when "int"
            @use_bclib ? "Bint.new(#{value})" : value
        when "float"
            @use_bclib ? "Bloat.new(#{value})" : value
        when "literal"
            f value
        when "array"
            res = "[#{value.map{|x| f x }.join(", ")}]"
            @use_bclib ? "Barray.new(#{res})" : res
        when "assignment"
            f(node["left"]) + " = " + f(node["right"])
        when "nil"
            "nil"
        when "binary_operation"
            if @use_bclib
                if node["operator"] == ".."
                    return "Brange.inclusive(#{f(node["left"])}, #{f(node["right"])})"
                end
                if node["operator"] == "..."
                    return "Brange.exclusive(#{f(node["left"])}, #{f(node["right"])})"
                end
            end
            "(" + f(node["left"]) + " " + node["operator"] + " " + f(node["right"]) + ")"
        when "call"
            # if node["args"].empty? && !node["parens"]
            #     return f(node["func"]) + ".try_call(" + node["args"].map{|x| f x }.join(", ") + ")"
            # end
            f(node["func"]) + ".call(" + node["args"].map{|x| f x }.join(", ") + ")"
        when "string"; "\"#{value}\""
        when "string_interpolate_start"; "\"#{value}#\{"
        when "string_interpolate_middle"; "}#{value}#\{"
        when "string_interpolate_end"; "}#{value}\""
        when "prefix"
            if %w"++ --".include?(node["operator"])
                operand = f(node["right"])
                return "(#{operand} #{node["operator"] == "++" ? "+" : "-"}= 1)"
            end
            "(#{node["operator"]} #{f(node["right"])})"
        when "postfix"
            if %w"++ --".include?(node["operator"])
                operand = f(node["left"])
                op = node["operator"] == "++" ? "+" : "-"
                return "((#{operand} #{op}= 1) #{op} -1)"
            end
            "<postfix>"
        when "amp_identifier"
            value
        when "amp_method"
            args = node["args"]
            if args.nil?
                return "%s.method(\"%s\")" % [
                    f(node["self"]),
                    node["name"]
                ]
            end
            "%s.%s(%s)" % [
                f(node["self"]),
                node["name"],
                node["args"].map{|x| f(x) }.join(", ")
            ]
        when "special_dollar"
            value
        when "func"
            args = node["args"].map{|x|f x}.join(", ")
            argc = node["args"].size
            var = rand_name()
            return "lambda{|*#{var}| #{f(node["body"])} }" if argc == 0
            return "lambda{|#{args}| #{f(node["body"])} }" if argc == 1
            # "lambda{|*#{var}| #{args} = #{var}; #{f(node["body"])} }"
            "lambda{|*#{var}| #{args} = #{var}.size == 1 ? #{var}[0] : #{var}; #{f(node["body"])} }"
        # when "func"
        #     # "|%s|%s" % [node["args"].map{|x|f x}.join(", "), f(node["body"])]
        #     args = node["args"]
        #     p args
        #     if args.empty?
        #         return "lambda{%s}" % f(node["body"])
        #     end
        #     "lambda{|%s|%s}" % [args.map{|x|f x}.join(", "), f(node["body"])]
        when "expressions"
            value.empty? ? "nil" : "(" + value.map{|x|f x}.join("; ") + ")"
        when "splat"
            # p node
            "*"  + f(value)
        when "ternary"
            "((#{f(node["condition"])}) ? (#{f(node["truthy"])}) : (#{f(node["falsy"])}))"
        when "bool"
            value
        when "index"
            "(#{f(node["self"])})[#{node["args"].map{|x|f x}.join(", ")}]"
        else
            puts "can't stringify"
            p node
            p "no"
        end
    end

    def rand_name
        "var_" + 20.times.map{rand 10}.join
    end
end