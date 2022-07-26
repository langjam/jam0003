import std.algorithm;
import std.string;
import raylib;
import dbasm;
import box;

Operation[] stack;
ubyte[string] registers;
ubyte[ulong] memory;
size_t[string] labels;
ulong memoryPointer;
size_t stackPointer;
ubyte comparisonResult;
Image buffer;

auto getRawValues(string item)
{
    auto toRemove = item.indexOf(";");
    if (toRemove > 0) item = item[0..toRemove + 1];

    return item.split(",");
}

Error buildStack(string[] lines)
{
    uint whileCount, foreverCount;
    string[] whileConditions;
    string[] loops;

    foreach(line; lines)
    {
        if (line.startsWith(";") || !line.length) continue;

        if (line.endsWith(":"))
        {
            auto label = line[0..line.length - 1].strip;

            if (labels.keys.canFind(label))
                return Error
                (
                    "Label name %s already in use at line %d.".format(label, labels[label] + 1),
                    lines.indexOf(line) + 1
                );

            labels[label] = lines.indexOf(line);
            continue;
        }

        auto items = line.strip.split(" ");

        if (items.length < 2)
            return Error
            (
                "expected a keyword command and its parameters, got %s instead".format(line),
                lines.indexOf(line) + 1
            );

        switch (items[0])
        {
            case "load":   stack ~= Operation(tokens.load);      break;
            case "add":    stack ~= Operation(tokens.add);       break;
            case "sub":    stack ~= Operation(tokens.substract)  break;
            case "inc":    stack ~= Operation(tokens.increment)  break;
            case "dec":    stack ~= Operation(tokens.decrement)  break;
            case "times":  stack ~= Operation(tokens.times)      break;
            case "div":    stack ~= Operation(tokens.division)   break;
            case "left":   stack ~= Operation(tokens.shiftLeft)  break;
            case "right":  stack ~= Operation(tokens.shiftRight) break;
            case "or":     stack ~= Operation(tokens.or)         break;
            case "and":    stack ~= Operation(tokens.and)        break;
            case "comp":   stack ~= Operation(tokens.compare)    break;
            case "jump":   stack ~= Operation(tokens.jump)       break;
            case "screen": stack ~= Operation(tokens.windowSize) break;
            case "data":   stack ~= Operation(tokens.data)       break;
            case "set":    stack ~= Operation(tokens.setRegister)break;

            case "while":

                // this ; makes sure there is no possible collision with user written labels
                labels[";while_%d".format(whileCount)] = lines.indexOf(line);
                //labels[";whend_%d".format(whileCount)] = 0; not enough time to implement proper while loops
                whileConditions ~= items[1];
                continue;

            case "forever":

                labels[";forever_%d".format(foreverCount++)] = lines.indexOf(line);
                continue;

            case "loop":

                if (loops.length == 0)
                    return Error
                    (
                        "End of loop statement found without a matching while or forever statement",
                        lines.indexOf(line) + 1;
                    );

                Operation op;

                if (loops[loops.length - 1].canFind("forever"))
                    op =
                    {
                        operator: tokens.jump,
                        registers: [loops[loops.length - 1]]
                    };

                else
                {
                    op =
                    {
                        operator: tokens.jump,
                        registers: [loops[loops.length - 1], whileConditions[whileConditions - 1]]
                    };

                    whileConditions = whileConditions[0..whileConditions.length - 1];
                }

                loops = loops[0..loops.length - 1];
                continue;

            default:
                return Error
                (
                    "invalid keyword \"%s\"".format(items[0]),
                    lines.indexOf(line) + 1
                );
        }

        foreach (rawValue; items[1].getRawValues)
        {
            if (rawValue.isNumeric && !rawValue.canFind("."))
                stack[stack.length - 1].values ~= AssemblyValue(rawValue);

            else stack[stack.length - 1].registers ~= rawValue;
        }
    }

    return Error("No error while scanning!", lines.length);
}

Error applyOperation(Operation op)
{
    final switch(op.operator)
    {
        case tokens.load:

            if (op.registers.length == 0)
                return Error("Load needs at least one register param", 1);

            if (op.registers.length < 2 && op.values.length == 0)
                return Error("Load needs at least two params", 1);

            if (op.values.length == 0)
            {
                for (auto i = 1; i < op.registers.length; ++i)
                {
                    if (!registers.keys.canFind(op.registers[i]))
                        return Error
                        (
                            "Register %s doesn't exist!".format(op.registers[i]),
                            1
                        );

                    registers[op.registers[i]] = registers[op.registers[0]];
                }
            }

            else if (op.values.length >= op.registers.length)
            {
                for (auto i = 0; i < op.registers.length; ++i)
                {
                    if (!registers.keys.canFind(op.registers[i]))
                        return Error
                        (
                            "Register %s doesn't exist!".format(op.registers[i]),
                            1
                        );

                    registers[op.registers[i]] = op.values[i].isMemoryLocation ? memory[op.values[i]] : cast(ubyte)op.values[i];
                }
            }

            else
            {
                foreach (register; op.registers)
                {
                    if (!registers.keys.canFind(register))
                        return Error
                        (
                            "Register %s doesn't exist!".format(register),
                            1
                        );

                    registers[register] = op.values[0];
                }
            }

            break;

        case tokens.add:

        if (op.registers.length == 0)
            return Error("add needs at least one register param", 1);


    }
}

void main()
{

}
