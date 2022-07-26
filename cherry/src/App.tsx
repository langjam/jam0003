import React, { useReducer } from "react";
import { InstructionLine } from "./components/InstructionLine";

import { Vm, Instruction, init as vmInit, update as vmUpdate } from "./vm/Vm";

type AppState = {
	// data: Data[];
	program: DraftInstruction[];
	endInstruction: number;
	vm: Vm;
};

export type DraftInstruction =
	| { instruction: Partial<Instruction>; editing: true; error?: string }
	| { instruction: Instruction; editing: false };

const init: AppState = {
	program: [
		{ instruction: { type: "add_i32_rir", r1: "a", i: 1, rr: "a" }, editing: false },
		{
			instruction: { type: "add_i32_rrr", r1: "a", r2: "a", rr: "a" },
			editing: false,
		},
		{
			instruction: { type: "add_i32_rrr", r1: "a", r2: "a", rr: "a" },
			editing: false,
		},
		{
			instruction: { type: "add_i32_rrr", r1: "a", r2: "a", rr: "a" },
			editing: false,
		},
		{
			instruction: { type: "add_i32_rrr", r1: "a", r2: "a", rr: "a" },
			editing: false,
		},
		{
			instruction: { type: "add_i32_rrr", r1: "a", r2: "a", rr: "a" },
			editing: false,
		},
		{
			instruction: { type: "add_i32_rrr", r1: "a", r2: "a", rr: "a" },
			editing: false,
		},
		{
			instruction: { type: "add_i32_rrr", r1: "a", r2: "a", rr: "a" },
			editing: false,
		},
		{ instruction: { type: "push_r", r: "a" }, editing: false },
		{ instruction: { type: "push_i", i: 256 }, editing: false },
		{ instruction: { type: "pop_r", r: "z" }, editing: false },
		{ instruction: { type: "putc_i", i: 72 }, editing: false },
		{ instruction: { type: "putc_i", i: 101 }, editing: false },
		{ instruction: { type: "putc_i", i: 108 }, editing: false },
		{ instruction: { type: "putc_i", i: 108 }, editing: false },
		{ instruction: { type: "putc_i", i: 111 }, editing: false },
		{ instruction: { type: "putc_i", i: 32 }, editing: false },
		{ instruction: { type: "putc_i", i: 102 }, editing: false },
		{ instruction: { type: "putc_i", i: 114 }, editing: false },
		{ instruction: { type: "putc_i", i: 105 }, editing: false },
		{ instruction: { type: "putc_i", i: 101 }, editing: false },
		{ instruction: { type: "putc_i", i: 110 }, editing: false },
		{ instruction: { type: "putc_i", i: 100 }, editing: false },
		{ instruction: { type: "putc_i", i: 33 }, editing: false },
		// { instruction: {}, editing: true },
	],
	endInstruction: 12,
	vm: vmInit,
};

type AppAction =
	| { type: "AddInstruction" }
	| { type: "EditInstruction"; index: number }
	| { type: "RemoveInstruction"; index: number }
	| { type: "RunInstruction"; index: number }
	| { type: "UpdateInstruction"; index: number; instruction: Instruction }
	| { type: "Play" }
	| { type: "Prev" }
	| { type: "Next" }
	| { type: "SetEndInstruction"; index: number };

function update(state = init, action: AppAction): AppState {
	switch (action.type) {
		case "AddInstruction":
			return {
				...state,
				program: [...state.program, { instruction: {}, editing: true }],
			};
		case "RunInstruction": {
			const draft = state.program[action.index];
			let vm = state.vm;

			if (draft && !draft.editing) {
				vm = vmUpdate(vm, draft.instruction);
			}

			return {
				...state,
				vm,
			};
		}
		case "UpdateInstruction": {
			const program = [...state.program];
			program[action.index] = {
				instruction: action.instruction,
				editing: false,
			};
			return {
				...state,
				program,
			};
		}
		case "EditInstruction": {
			const program = [...state.program];
			program[action.index] = {
				instruction: program[action.index]!.instruction,
				editing: true,
			};
			return {
				...state,
				program,
			};
		}
		case "RemoveInstruction": {
			const program = [...state.program];
			program.splice(action.index, 1);
			return {
				...state,
				program,
			};
		}
		case "SetEndInstruction":
			return {
				...state,
				endInstruction: action.index,
				vm: (
					state.program
						.slice(0, action.index)
						.filter((draft) => !draft.editing)
						.map((draft) => draft.instruction) as Instruction[]
				).reduce(vmUpdate, vmInit),
			};
		case "Play":
			return {
				...state,
				endInstruction: state.program.length,
				vm: (
					state.program
						.filter((draft) => !draft.editing)
						.map((draft) => draft.instruction) as Instruction[]
				).reduce(vmUpdate, vmInit),
			};
		case "Prev": {
			const endInstruction = Math.max(state.endInstruction - 1, 0);

			return {
				...state,
				endInstruction,
				vm: (
					state.program
						.slice(0, endInstruction)
						.filter((draft) => !draft.editing)
						.map((draft) => draft.instruction) as Instruction[]
				).reduce(vmUpdate, vmInit),
			};
		}
		case "Next": {
			const endInstruction = Math.min(
				state.endInstruction + 1,
				state.program.length - 1,
			);

			return {
				...state,
				endInstruction,
				vm: (
					state.program
						.slice(0, endInstruction)
						.filter((draft) => !draft.editing)
						.map((draft) => draft.instruction) as Instruction[]
				).reduce(vmUpdate, vmInit),
			};
		}
		default:
			return state;
	}
}

export function App() {
	const [state, dispatch] = useReducer(update, init);

	return (
		<div className="App">
			<div className="editor">
				{/* <h1>Data</h1> */}
				{/* <button onClick={() => dispatch({ type: "RunInstruction" })}>Run</button> */}
				{/* <h1>Instructions</h1> */}
				<div className="instructions">
					{state.program.map((draft, index) => (
						<InstructionLine
							key={index}
							draft={draft}
							onRun={() => dispatch({ type: "RunInstruction", index })}
							onSave={(instruction) =>
								dispatch({
									type: "UpdateInstruction",
									index,
									instruction,
								})
							}
							onSelect={() =>
								dispatch({ type: "SetEndInstruction", index: index + 1 })
							}
						/>
					))}
				</div>
				<div className="instructions-bar">
					<button onClick={() => dispatch({ type: "AddInstruction" })}>
						Add
					</button>
					<button onClick={() => dispatch({ type: "Prev" })}>Prev</button>
					<button onClick={() => dispatch({ type: "Next" })}>Next</button>
					<button onClick={() => dispatch({ type: "Play" })}>Play</button>
				</div>
			</div>
			<div className="machine-panel">
				<h6 className="section-header">Registers</h6>
				<div className="machine">
					<code>a</code>
					<code className="rd">{state.vm.a}</code>
					<code>b</code>
					<code className="rd">{state.vm.b}</code>
					<code>c</code>
					<code className="rd">{state.vm.c}</code>
					<code>d</code>
					<code className="rd">{state.vm.d}</code>
					<code>w</code>
					<code className="rd">{state.vm.w}</code>
					<code>x</code>
					<code className="rd">{state.vm.x}</code>
					<code>y</code>
					<code className="rd">{state.vm.y}</code>
					<code>z</code>
					<code className="rd">{state.vm.z}</code>
				</div>

				<h6 className="section-header">Stack</h6>
				<div className="stack">
					{state.vm.stack.map((value, index) => (
						<div>
							<code key={index}>{value}</code>
						</div>
					))}
				</div>

				<h6 className="section-header">Output</h6>
				<pre className="machine-output">
					{new TextDecoder().decode(new Uint8Array(state.vm.output)) ??
						"No output"}
				</pre>
			</div>
		</div>
	);
}
