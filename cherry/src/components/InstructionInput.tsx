import React, { ComponentPropsWithoutRef } from "react";

export function InstructionInput(props: ComponentPropsWithoutRef<"select">) {
	const { ...attrs } = props;
	return (
		<select {...attrs}>
			<option value="add_i32_rir">add_i32_rir</option>
			<option value="add_i32_rrr">add_i32_rrr</option>
		</select>
	);
}
