import { Type, $interface, $number } from "succulent";
import { $Register } from "../types";

export type putc_r = Type<typeof $putc_r>;
export const $putc_r = $interface({
	type: "putc_r",
	r: $Register,
});

export type putc_i = Type<typeof $putc_i>;
export const $putc_i = $interface({
	type: "putc_i",
	i: $number,
});

export type push_r = Type<typeof $push_r>;
export const $push_r = $interface({
	type: "push_r",
	r: $Register,
});

export type push_i = Type<typeof $push_i>;
export const $push_i = $interface({
	type: "push_i",
	i: $number,
});

export type pop_r = Type<typeof $pop_r>;
export const $pop_r = $interface({
	type: "pop_r",
	r: $Register,
});
