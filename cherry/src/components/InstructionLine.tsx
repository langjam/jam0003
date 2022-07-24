import React, { useCallback, useRef, useState } from "react";
import { is } from "succulent";

import type { DraftInstruction } from "../App";
import { $Instruction, type Instruction } from "../vm/Vm";
import { IntegerInput } from "./IntegerInput";
import { InstructionInput } from "./InstructionInput";
import { RegisterInput } from "./RegisterInput";
import "./InstructionLine.css";

interface InstructionLineProps {
	draft: DraftInstruction;
	onRun: () => void;
	onSave: (instruction: Instruction) => void;
	onSelect: () => void;
}

function serializeInstruction(instruction: Instruction) {
	switch (instruction.type) {
		case "add_i32_rir":
			return `add.i32 ${instruction.r1} ${instruction.i} ${instruction.rr}`;
		case "add_i32_rrr":
			return `add.i32 ${instruction.r1} ${instruction.r2} ${instruction.rr}`;
		case "push_r":
			return `push ${instruction.r}`;
		case "push_i":
			return `push ${instruction.i}`;
		case "pop_r":
			return `pop ${instruction.r}`;
		case "putc_i":
			return `putc '${String.fromCharCode(instruction.i)}'`;
		default:
			return "[unrecognized]";
	}
}

export function InstructionLine(props: InstructionLineProps) {
	const { draft, onRun, onSave, onSelect, ...attrs } = props;
	const ref = useRef<HTMLFormElement>(null);

	const instructionName =
		(new FormData(ref.current ?? undefined).get("instruction") as string) ??
		"add_i32_rrr";

	const [savingEnabled, setSavingEnabled] = useState(false);

	const onChange = useCallback(() => {
		const instruction = Object.fromEntries(new FormData(ref.current ?? undefined));
		console.log(is(instruction, $Instruction), instruction);
		setSavingEnabled(is(instruction, $Instruction));
	}, []);

	return draft.editing ? (
		<form ref={ref} onChange={onChange} className="form">
			<InstructionInput name="type" />
			<RegisterInput name="r1" />
			{instructionName.endsWith("_rir") ? (
				<IntegerInput name="i" />
			) : (
				<RegisterInput name="r2" />
			)}
			<RegisterInput name="rr" />
			{savingEnabled && (
				<button
					type="button"
					onClick={() =>
						onSave(
							Object.fromEntries(new FormData(ref.current!)) as Instruction,
						)
					}
				>
					Save
				</button>
			)}
		</form>
	) : (
		<div className="line" onClick={onSelect}>
			<code className="type">({draft.instruction.type})</code>
			<code>{serializeInstruction(draft.instruction)}</code>
		</div>
	);
}
