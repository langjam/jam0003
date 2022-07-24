import { CharStreams, CommonTokenStream } from 'antlr4ts';
import { program } from 'commander';
import { ProgramVisitor, resolveComponent } from './parser';
import { langLexer } from './parser/langLexer';
import { langParser } from './parser/langParser';
import { simulate } from './simulate';
import * as fs from 'fs';
import * as readline from 'readline';

program
    .argument("<file>", "The file to interpret")
    .option("-n, --non-interactive <component> <inputs...>", "Run the component in non-interactive mode with the given inputs");

function main() {
    let command = program.parse(process.argv);
    let file = command.args[0];
    let option: string[] = command.opts().nonInteractive;
    
    // Create the lexer and parser
    let inputStream = CharStreams.fromString(fs.readFileSync('./example.cc').toString('utf-8'));
    let lexer = new langLexer(inputStream);
    let tokenStream = new CommonTokenStream(lexer);
    let parser = new langParser(tokenStream);

    // Parse the input, where `compilationUnit` is whatever entry point you defined
    let tree = parser.program();
    let unresolvedComponents = new ProgramVisitor().visit(tree);

    if (parser.numberOfSyntaxErrors > 0) {
        console.log("Syntax error");
        return;
    }

    if (option) {
        let component = option[0];
        let inputs = option.slice(1).map(x => isNumeric(x) ? parseInt(x) : x);
        if (inputs.find(x => typeof x != "number" && x != "push" && x != "pull")) {
            console.log("Inputs must be either 'push' or 'pull' or a number");
            return;
        }

        let resolvedComponent = resolveComponent(unresolvedComponents, component);
        let states = simulate(resolvedComponent.inputs, inputs as ("push" | "pull" | number)[]);

        console.log("Outputs: ", resolvedComponent.outputs.map(x => x.state));
        console.log("After execution states were: ");
        console.log(states);
    } else {
        console.log("\x1b[32mclockwork computation\x1b[0m")
        console.log("¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯")
        console.log("[ Welcome to interactive mode. The following components were parsed: ]");
        unresolvedComponents.forEach(x => console.log("    - ",x.name));

        console.log("\nEnter a component name and inputs to run it in interactive mode. Enter 'exit' to exit.");
        console.log("Example: my_component(push, 1)\n")

        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
            prompt: '> '
        });

        rl.prompt();

        rl.on('line', (answer) => {
            if (answer == "exit") {
                console.log("Bye!");
                process.exit(0);   
            } else {
                let splitted = answer.split('(');
                if (splitted.length != 2 || answer.slice(-1) != ')') {
                    console.log("Invalid input");
                    return;
                }

                let component = splitted[0];
                let inputs = splitted[1].slice(0, -1).split(',').map(x => isNumeric(x) ? parseInt(x.trim()) : x.trim());
                if (inputs.find(x => typeof x != "number" && x != "push" && x != "pull")) {
                    console.log("Inputs must be either 'push' or 'pull' or a number");
                    return;
                }

                let resolvedComponent = resolveComponent(unresolvedComponents, component);
                let states = simulate(resolvedComponent.inputs, inputs as ("push" | "pull" | number)[]);

                console.log("Outputs: ", resolvedComponent.outputs.map(x => x.state));
                console.log("After execution states were: ");
                console.log(states);
                console.log();

                rl.prompt();
            }
        }).on('close', () => {
            console.log('Bye!');
            process.exit(0);
        });
    }
}
function isNumeric(str: string) {
    // @ts-ignore
    return !isNaN(str) && // use type coercion to parse the _entirety_ of the string (`parseFloat` alone does not do this)...
           !isNaN(parseFloat(str)) // ...and ensure strings of whitespace fail
}


main();