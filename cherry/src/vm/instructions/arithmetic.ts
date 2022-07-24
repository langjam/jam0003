import { Type, $interface, $number } from "succulent";
import { $Register } from "../types";

export type add_i32_rir = Type<typeof $add_i32_rir>;
export const $add_i32_rir = $interface({
	type: "add_i32_rir",
	r1: $Register,
	i: $number,
	rr: $Register,
});

export type add_i32_rrr = Type<typeof $add_i32_rrr>;
export const $add_i32_rrr = $interface({
	type: "add_i32_rrr",
	r1: $Register,
	r2: $Register,
	rr: $Register,
});
