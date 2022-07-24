import { Type, union } from "succulent";

import { $add_i32_rir, $add_i32_rrr } from "./instructions/arithmetic";
import { $putc_r, $putc_i, $push_r, $push_i, $pop_r } from "./instructions/io";

export type Instruction = Type<typeof $Instruction>;
export const $Instruction = union(
	$add_i32_rir,
	$add_i32_rrr,
	$putc_r,
	$putc_i,
	$push_r,
	$push_i,
	$pop_r,
);

export type Vm = {
	a: number;
	b: number;
	c: number;
	d: number;
	w: number;
	x: number;
	y: number;
	z: number;
	stack: number[];
	output: number[];
};

export const init: Vm = {
	a: 0,
	b: 0,
	c: 0,
	d: 0,
	w: 0,
	x: 0,
	y: 0,
	z: 0,
	stack: [0],
	output: [
		62, 32, 115, 116, 97, 114, 116, 105, 110, 103, 32, 99, 104, 101, 114, 114, 121,
		45, 118, 109, 46, 46, 46, 10, 10,
	],
};

export function update(state = init, message: Instruction) {
	switch (message.type) {
		case "add_i32_rir":
			return {
				...state,
				[message.rr]: state[message.r1] + message.i,
			};
		case "add_i32_rrr":
			return {
				...state,
				[message.rr]: state[message.r1] + state[message.r2],
			};
		case "putc_r":
			return {
				...state,
				output: [...state.output, state[message.r]],
			};
		case "putc_i":
			return {
				...state,
				output: [...state.output, message.i],
			};
		case "push_r":
			return {
				...state,
				stack: [...state.stack, state[message.r]],
			};
		case "push_i":
			return {
				...state,
				stack: [...state.stack, message.i],
			};
		case "pop_r":
			return {
				...state,
				[message.r]: state.stack.at(-1),
				stack: state.stack.slice(0, -1),
			};
		default:
			return state;
	}
}
