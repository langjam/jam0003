import { Part, Connection, UnresolvedGear, Rod, Component } from "./types";

type PropogateConnection = {
    leftName: string;
    connection: Connection;
    state: number | "push" | "pull";
    teeth?: number;
}

function mod(n: number, m: number): number {
    return ((n % m) + m) % m;
}

export function simulate(inputComponents: Part[], inputs: ("push" | "pull" | number)[]): Map<string, "push" | "pull" | number> {
    const states = new Map<string, "push" | "pull" | number>();
    const stack = new Map<Part | Component, PropogateConnection[]>();

    if (inputComponents.length != inputs.length) {
        throw new Error("inputs and components must be the same length");
    } 

    for (let i = 0; i < inputComponents.length; i++) {
        const component = inputComponents[i];
        const input = inputs[i];

        if (!(["push", "pull"] as any[]).includes(input) && typeof input != "number") {
            throw new Error("input must be either 'push' or 'pull' or a number");
        }

        if (component.kind == "gear" && typeof input != "number") {
            throw new Error("input must be a number for gears");
        } else if (component.kind == "rod" && (input != "pull" && input != "push")) {
            throw new Error("input must be 'pull' or 'push' for rods");
        }

        for (const connection of component.connectedRight) {
            const key = connection.part;
            const val = {
                leftName: component.name,
                connection: connection,
                state: input,
                teeth: component.kind == "gear" ? component.teeth : undefined,
            };
            stack.has(key) ? stack.get(key)!.push(val) : stack.set(key, [val]);
        }

        component.state = input;
        states.set(component.name, input);
    }

    while (stack.size > 0) {
        let entries = stack.entries();
        const [component, props]: [Part | Component, PropogateConnection[]] = stack.entries().next().value;
        stack.delete(component);

        if (component.kind == "component") {
            if (props.length < component.inputs.length) {
                // Not enough inputs.
                stack.set(component, props);
                continue;
            }
            inputs = new Array(component.inputs.length);
            // This must be a use
            for (const prop of props) {
                if (prop.connection.kind != "use") { throw new Error("Unexpected connection kind"); }
                inputs[prop.connection.parameterIndex] = prop.state;
            }
            simulate(component.inputs, inputs);
            
            for (let i = 0; i < component.outputs.length; i++) {
                component.output_map![i].state = component.outputs[i].state;
                states.set(component.output_map![i].name, component.outputs[i].state);

                const actualComponent = component.output_map![i];
                // Propogate those connections baby
                for (const connection of actualComponent.connectedRight) {
                    const key = connection.part;
                    const val = {
                        leftName: actualComponent.name,
                        connection: connection,
                        state: actualComponent.state,
                        teeth: actualComponent.kind == "gear" ? actualComponent.teeth : undefined,
                    }
                    if (stack.has(key)) {
                        stack.get(key)!.push(val);
                    } else {
                        stack.set(key, [val]);
                    }
                }
            }

            continue;
        }

        let forcedState: number | "push" | "pull" | null = null;
        for (const prop of props) {
            if (prop.connection.kind == "rod-rod") {
                if (prop.connection.rodAttachment == "attach") {
                    component.state = prop.state;
                    verifyConsistency(forcedState, component.state);
                    forcedState = component.state;
                } else if (prop.connection.rodAttachment == "push" && prop.state == "push") {
                    component.state = "push";
                    verifyConsistency(forcedState, component.state);
                    forcedState = component.state;
                } else if (prop.connection.rodAttachment == "pull" && prop.state == "pull") {
                    component.state = "pull";
                    verifyConsistency(forcedState, component.state);
                    forcedState = component.state;
                }
            } else if (prop.connection.kind == "gear-rod") {
                if (component.kind == "gear") {
                    // Rod moving a gear
                    if (prop.state == "push") {
                        component.state = (prop.connection.gearOffset + component.teeth / 2) % component.teeth;
                    } else {
                        component.state = prop.connection.gearOffset;
                    }
                } else {
                    // Gear moving a rod
                    const rodPosition = (prop.connection.gearOffset + (prop.state as number)) % prop.teeth!;
                    if (rodPosition == 0) {
                        component.state = "pull";
                    } else if (rodPosition == prop.teeth! / 2) {
                        component.state = "push";
                    } else if (rodPosition < prop.teeth! / 2) {
                        component.state = "pull";
                        //throw new Error(`Rod not fully pulled or pushed! Expected 0 or ${prop.teeth! / 2}, got ${rodPosition}`);
                    } else if (rodPosition > prop.teeth! / 2) {
                        component.state = "push";
                        //throw new Error(`Rod not fully pulled or pushed! Expected 0 or ${prop.teeth! / 2}, got ${rodPosition}`);
                    }
                }
                verifyConsistency(forcedState, component.state);
                forcedState = component.state;
            } else if (prop.connection.kind == "gear-gear") {
                // Gear moving a gear
                // Note that no consistency check is done here because we assume these are one way ratchet-y gears
                if (component.kind != "gear" || typeof prop.state != "number") { 
                    console.log(states);
                    throw new Error("unreachable"); 
                }
                component.state = mod((component.state - prop.state), component.teeth); // backwards direction remember
            } else {
                // Use connection
                throw new Error("Unexpected connection kind");
            }
        }

        if (forcedState == null && component.kind == "rod" && component.spring != "none") {
            component.state = component.spring;
        }

        states.set(component.name, component.state);

        // Propogate
        for (const connection of component.connectedRight) {
            const key = connection.part;
            const val = {
                leftName: component.name,
                connection: connection,
                state: component.state,
                teeth: component.kind == "gear" ? component.teeth : undefined,
            }
            if (stack.has(key)) {
                stack.get(key)!.push(val);
            } else {
                stack.set(key, [val]);
            }
        }
    }

    return states;
}



function verifyConsistency(forcedState: string | number | null, newState: "push" | "pull" | number) {
    if (forcedState != null && forcedState != newState) {
        throw new Error("Connection is not consistent. Forced: " + forcedState + " New: " + newState);
    }
}
