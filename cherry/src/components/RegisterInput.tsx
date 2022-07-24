import React, { ComponentPropsWithoutRef } from "react";

export function RegisterInput(props: ComponentPropsWithoutRef<"select">) {
	const { ...attrs } = props;
	return (
		<select {...attrs}>
			<option value="a">a</option>
			<option value="b">b</option>
			<option value="c">c</option>
			<option value="d">d</option>
			<option value="e">e</option>
			<option value="f">f</option>
			<option value="g">g</option>
			<option value="h">h</option>
		</select>
	);
}
