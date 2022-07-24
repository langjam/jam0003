import sys
from enum import Enum

args = sys.argv

if len(sys.argv) < 2:
    print("Input assembly line file path!")
    sys.exit()

spec_file = sys.argv[1]
spec_file = open(spec_file)

machines = []
machine_locations = {}
machine_locations_reversed = {}
machine_output_directions = {}

searching_machines = False

machine_definitions = {}

class Stage(Enum):
    CONFIGURATION = 0
    LAYOUT = 1
    MACHINES = 2

machines_inputs = {}
machine_outputs = {}

def parse(raw):
    parsed = []

    for input in raw:
        if input:
            if input[0] == '"':
                input = input[1 : len(input) - 1]
            else:
                try:
                    int_value = int(input)
                    input = int_value
                except ValueError:
                    pass
        parsed.append(input)

    return parsed

for row, line in enumerate(spec_file.readlines()):
    line = line.replace('\n', '')
    if "machine " in line and not line.startswith('#'):
        name = line[line.index("machine ") + 8 : line.index(':')]
        input_count = 0
        if '(' in name:
            inputs = name[name.index('(') + 1 : name.index(')')].split(',')
            input_count = len(inputs)
            name = name[0 : name.index('(')]

            new_inputs = [string.strip() for string in inputs if string]

            if len(new_inputs) == 0:
                print("Machine " + name + " has empty ()!")
                exit()

            machines_inputs[name] = list(new_inputs)
            for input in new_inputs:
                if not input in machine_outputs:
                    machine_outputs[input] = []

                machine_outputs[input].append(name)
        else:
            machines_inputs[name] = []

        if name in machines:
            print("Machine " + name + " has duplicate definitions!")
            exit()
        machines.append(name)

        transformation = line.split(':')[1].split(' ')[1].strip()

        inputs = []

        parsed = []
        raw = []
        buffer = ""
        in_quotes = False
        for character in ' '.join(line.split(':')[1].split(' ')[2:]):
            if character == '"':
                in_quotes = not in_quotes

            if character == ' ' and not in_quotes:
                raw.append(buffer)
                buffer = ""
            else:
                buffer += character

        if buffer:
            raw.append(buffer)

        inputs.extend(parse(raw))

        modifiers = []

        for modifier in line.split(' '):
            if modifier == "machine":
                break

            modifiers.append(modifier)
            
        machine_definitions[name] = (input_count, transformation, inputs, modifiers)

machine_inputs_available = {}
run_count = {}

def run_transformation(machine, id, inputs):
    match id:
        case "add":
            return add_(inputs[0], inputs[1])
        case "subtract":
            return subtract_(inputs[0], inputs[1])
        case "equal":
            return equal_(inputs[0], inputs[1])
        case "print":
            return print_(inputs[0])
        case "passthrough":
            return passthrough_(inputs[0])
        case "repeat":
            return repeat_(machine, inputs[0], inputs[1])
        case "repeat_state":
            return repeat_state_(machine, inputs)
        case "if":
            return if_(machine, inputs[0], inputs[1])
        case "nothing":
            return nothing_()
        case thing:
            print("Undefined transformation '" + thing + "'!")
            exit()

exiting = False
output_uses = []
dependencies = {}
output_cache = {}

def add_(value1, value2):
    return value1 + value2

def subtract_(value1, value2):
    return value1 - value2

def equal_(value1, value2):
    return value1 == value2

def print_(value):
    print(value)

def passthrough_(value):
    return value

def nothing_():
    pass

def repeat_(caller_machine, machine, count):
    if not machine in machines:
        print("repeat not supported with transformations")
        exit()

    if not "external" in machine_definitions[machine][3]:
        print("Machine called via transformation must be external!")
        exit()

    for i in range(0, count):
        if len(machines_inputs[machine]) > 0:
            for machine_new in machines_inputs[machine]:
                output_uses.append(machine_new)
                machine_inputs_available[machine_new].append([i])
        else:
            machine_inputs_available[machine].append([i])

        output_uses.append(machine)

    list = [machine]
    if len(machines_inputs[machine]) > 0:
        list.append(machines_inputs[machine][0])

    #print(list)

    dependencies[caller_machine] = list

def if_(caller_machine, condition, machine):
    if not machine in machines:
        print("repeat not supported with transformations")
        exit()

    if not "external" in machine_definitions[machine][3]:
        print("Machine called via transformation must be external!")
        exit()

    if condition:
        if len(machines_inputs[machine]) > 0:
            for machine_new in machines_inputs[machine]:
                output_uses.append(machine_new)

        machine_inputs_available[machine].append([0])
        output_uses.append(machine)

    list = [machine]
    if len(machines_inputs[machine]) > 0:
        list.append(machines_inputs[machine][0])

    dependencies[caller_machine] = list

def repeat_state_(caller_machine, inputs):
    inputs = list(inputs)
    transformation = inputs[0]

    if transformation in machines:
        print("repeat_counter not supported with machines")
        exit()

    #print(transformation)
    #print(inputs)
    state_index = inputs.index("{0}")
    state_previous_index = inputs.index("{-1}")
    counter_index = -1
    if "<0>" in inputs:
        counter_index = inputs.index("<0>")

    initial = inputs[len(inputs) - 1]
    state = initial
    state_previous = state

    for i in range(0, inputs[len(inputs) - 2]):
        inputs[state_index] = state
        inputs[state_previous_index] = state_previous

        if not counter_index == -1:
            inputs[counter_index] = i

        state_previous = state
        state = run_transformation(caller_machine, inputs[0], inputs[1 : len(inputs) - 2])

    return state

def run_machine(name):
    global exiting
    #run_count[name] = 0
    definition = machine_definitions[name]
    if True:
        if name in output_cache:
            done = True
            for dependency in dependencies[name]:
                #print(machine_inputs_available[dependency])
                for input in machine_inputs_available[dependency]:
                    if not input == None:
                        for thing in input:
                            if not thing == None:
                                #print(dependency)
                                done = False

            if done:
                output = output_cache[name]
                if name in machine_outputs:
                    next_machines = machine_outputs[name]
                    for next_machine in next_machines:
                        if len(machine_inputs_available[next_machine]) < run_count[name] + 1:
                            machine_inputs_available[next_machine].append([None] * machine_definitions[next_machine][0])

                        if output == None:
                            output = "nothing"

                        machine_inputs_available[next_machine][run_count[name]][machines_inputs[next_machine].index(name)] = output
                        #print("test")
                    del output_cache[name]
                    del dependencies[name]

                if "product" in definition[3]:
                    exiting = True

                run_count[name] += 1

                if name in output_uses:
                    output_uses.remove(name)
            else:
                return

        if name in output_uses or (not name in machine_outputs and not "external" in definition[3]):
            run = False

            actual_inputs = [None] * definition[0]
            if "input" in definition[3]:
                actual_inputs = [None] * (len(args) - 2)
                for index, actual_input in enumerate(parse(args[2:])):
                    actual_inputs[index] = actual_input

            if definition[0] > 0 or "external" in machine_definitions[name][3]:
                for index, input in enumerate(machine_inputs_available[name]):
                    if input:
                        is_valid = True
                        for input0 in input:
                            if input0 == None:
                                is_valid = False

                        if is_valid:
                            actual_inputs = machine_inputs_available[name][index]
                            machine_inputs_available[name][index] = None
                            run = True
                            break
            else:
                run = True

            #print(name + " " + str(run))

            if run:
                transformation_inputs = list(definition[2])
                for index, input in enumerate(list(transformation_inputs)):
                    if isinstance(input, str) and input.startswith("["):
                        index0 = int(input[1 : len(input) - 1])
                        if index0 >= len(actual_inputs) and "input" in definition[3]:
                            print("Assembly line requires at least " + str(index0 + 1) + " parameters, given " + str(len(actual_inputs)))
                            exit()

                        transformation_inputs[index] = actual_inputs[index0]

                is_transformation = True

                output = None
                if is_transformation:
                    output = run_transformation(name, definition[1], transformation_inputs)

                if name in dependencies:
                    output_cache[name] = output
                    return

                if name in machine_outputs:
                    next_machines = machine_outputs[name]
                    for next_machine in next_machines:
                        if len(machine_inputs_available[next_machine]) < run_count[name] + 1:
                            machine_inputs_available[next_machine].append([None] * machine_definitions[next_machine][0])

                        if output == None:
                            output = "nothing"

                        if machine_inputs_available[next_machine][run_count[name]] == None:
                            machine_inputs_available[next_machine][run_count[name]] = [None] * len(machines_inputs[next_machine])

                        machine_inputs_available[next_machine][run_count[name]][machines_inputs[next_machine].index(name)] = output
                    
                    #print(name)
                else:
                    if "product" in definition[3]:
                        exiting = True

                #print(run_count[name])
                #if run
                #machine_inputs_available[name][run_count[name]] = None

                run_count[name] += 1
                #print(run_count[name])
            else:
                add = True
                for input in machines_inputs[name]:
                    if input in output_uses:
                        add = False 

                if add:
                    output_uses.extend(machines_inputs[name])

            if name in output_uses:
                output_uses.remove(name)

            #print(output_uses)
            #print(name)

has_product_machine = False

for machine in machines:
    machine_inputs_available[machine] = []
    if "product" in machine_definitions[machine][3]:
        has_product_machine = True

    for input in machines_inputs[machine]:
        if not input in machines:
            print("Machine " + input + " not defined!")
            exit()

        if "external" in machine_definitions[input][3] and not "external" in machine_definitions[machine][3]:
            print("External cannot be used without transformation!")
            exit()

    run_count[machine] = 0

if not has_product_machine:
    print("No product machine defined!")
    exit()

while not exiting:
    for machine in machines:
        run_machine(machine)
