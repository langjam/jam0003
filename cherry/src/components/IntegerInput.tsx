import React, { ComponentPropsWithoutRef } from "react";

export function IntegerInput(props: ComponentPropsWithoutRef<"select">) {
	const { ...attrs } = props;
	return (
		<select {...attrs}>
			<option value="0">0</option>
			<option value="1">1</option>
			<option value="2">2</option>
			<option value="3">3</option>
			<option value="4">4</option>
			<option value="5">5</option>
			<option value="6">6</option>
			<option value="7">7</option>
		</select>
	);
}
