#include "common.h"
#include "vm.h"
#include "vmintrin.h"
#include "wasm.h"

static
int vm_pop(VM *vm, float *o) {
	if (!vm->sk.values_len) return 1;
	*o = vm->sk.values[--vm->sk.values_len];

	return 0;
}

static
int vm_push(VM *vm, float v) {
	if (vm->sk.values_len >= STACK_MAX - 1) {
		putstr(WASM_STDOUT, "VM: stack overflow");
		return 1;
	}

	vm->sk.values[vm->sk.values_len++] = v;

	return 0;
}

static
int vm_instr_add(VM *vm) {
	float a, b;
	if (vm_pop(vm, &a) || vm_pop(vm, &b))
		return 1;
	
	return vm_push(vm, b + a);
}

static
int vm_instr_mul(VM *vm) {
	float a, b;
	if (vm_pop(vm, &a) || vm_pop(vm, &b))
		return 1;
	
	return vm_push(vm, b + a);
}

static
int vm_instr_sub(VM *vm) {
	float a, b;
	if (vm_pop(vm, &a) || vm_pop(vm, &b))
		return 1;
	
	return vm_push(vm, b - a);
}

static
int vm_instr_div(VM *vm) {
	float a, b;
	if (vm_pop(vm, &a) || vm_pop(vm, &b))
		return 1;

	if (a == 0) {
		putstr(WASM_STDOUT, "VM: error: division by zero\n");
		return 1;
	}
	
	return vm_push(vm, b / a);
}

static
int vm_instr_mod(VM *vm) {
	float a, b;
	if (vm_pop(vm, &a) || vm_pop(vm, &b))
		return 1;

	if (a == 0) {
		putstr(WASM_STDOUT, "VM: error: division by zero\n");
		return 1;
	}
	
	return vm_push(vm, (int)b % (int)a);
}

int vm_run(VM *vm) {
	while (vm->ip < vm->prog.instrs_len) {
		const uint arg = vm->prog.instrs[vm->ip].argument;

		switch (vm->prog.instrs[vm->ip].type) {
		case InstrType_Add:
			if (vm_instr_add(vm)) return 1;
			break;
		case InstrType_Mul:
			if (vm_instr_mul(vm)) return 1;
			break;
		case InstrType_Sub:
			if (vm_instr_sub(vm)) return 1;
			break;
		case InstrType_Div:
			if (vm_instr_div(vm)) return 1;
			break;
		case InstrType_Mod:
			if (vm_instr_mod(vm)) return 1;
			break;

		case InstrType_Pop:
			float _;
			for (int i=0; i < arg && vm->sk.values_len; i++)
				vm_pop(vm, &_);
			break;

		case InstrType_Imm:
			if (vm_push(vm, arg)) return 1;
			break;

		case InstrType_Load:
			if (vm_push(vm, vm->regs[arg])) return 1;
			break;

		case InstrType_Store:
			float a;
			if (vm_pop(vm, &a)) return 1;

			vm->regs[arg] = a;
			break;

		default:
			putstr(WASM_STDOUT, "VM: unknown instruction\n");
			return 1;
		}
	}

	return 0;
}

VM vm_init(Program prog) {
	VM vm = VM{};

	vm.prog = prog;

	return vm;
}
