import { Type, union } from "succulent";

export type Register = Type<typeof $Register>;
export const $Register = union("a", "b", "c", "d", "w", "x", "y", "z");
