#include "debug.h"
#include "common.h"
#include "vm.h"
#include "vmintrin.h"
#include "wasm.h"

static
int vm_pop(VM *vm, float *o) {
	CHECKOUT(!vm->sk.values_len);
	*o = vm->sk.values[--vm->sk.values_len];

	return 0;
}

static
int vm_push(VM *vm, float v) {
	if (vm->sk.values_len >= STACK_MAX - 1) {
		putstr(WASM_STDOUT, "VM: stack overflow");
		CHECKOUT(1)
	}

	vm->sk.values[vm->sk.values_len++] = v;

	return 0;
}

static
int vm_instr_add(VM *vm) {
	float a, b;
	CHECKOUT(vm_pop(vm, &a) || vm_pop(vm, &b));
	
	return vm_push(vm, b + a);
}

static
int vm_instr_mul(VM *vm) {
	float a, b;
	CHECKOUT(vm_pop(vm, &a) || vm_pop(vm, &b));
	
	return vm_push(vm, b + a);
}

static
int vm_instr_sub(VM *vm) {
	float a, b;
	CHECKOUT(vm_pop(vm, &a) || vm_pop(vm, &b));
	
	return vm_push(vm, b - a);
}

static
int vm_instr_div(VM *vm) {
	float a, b;
	CHECKOUT(vm_pop(vm, &a) || vm_pop(vm, &b));

	if (a == 0) {
		putstr(WASM_STDOUT, "VM: error: division by zero\n");
		CHECKOUT(1)
	}
	
	return vm_push(vm, b / a);
}

static
int vm_instr_mod(VM *vm) {
	float a, b;
	CHECKOUT(vm_pop(vm, &a) || vm_pop(vm, &b));

	if (a == 0) {
		putstr(WASM_STDOUT, "VM: error: division by zero\n");
		CHECKOUT(1);
	}
	
	return vm_push(vm, (int)b % (int)a);
}

int vm_run(VM *vm) {
	while (vm->ip < vm->prog.instrs_len) {
		const uint arg = vm->prog.instrs[vm->ip].argument;

		switch (vm->prog.instrs[vm->ip].type) {
		case InstrType_Add:
			CHECKOUT(vm_instr_add(vm));
			break;
		case InstrType_Mul:
			CHECKOUT(vm_instr_mul(vm));
			break;
		case InstrType_Sub:
			CHECKOUT(vm_instr_sub(vm));
			break;
		case InstrType_Div:
			CHECKOUT(vm_instr_div(vm));
			break;
		case InstrType_Mod:
			CHECKOUT(vm_instr_mod(vm));
			break;

		case InstrType_Pop:
			float _;
			for (int i=0; i < arg && vm->sk.values_len; i++)
				vm_pop(vm, &_);
			break;

		case InstrType_Imm:
			CHECKOUT(vm_push(vm, arg));
			break;

		case InstrType_Load:
			CHECKOUT(vm_push(vm, vm->regs[arg]));
			break;

		case InstrType_Store:
			float a;
			CHECKOUT(vm_pop(vm, &a));

			vm->regs[arg] = a;
			break;

		default:
			putstr(WASM_STDOUT, "VM: unknown instruction\n");
			return 1;
		}
	}

	return 0;
}

void vm_run_for_pixel(VM *vm, u8 screen[256][256], sint x, sint y) {
	vm->regs[Reg_X] = x;
	vm->regs[Reg_Y] = y;

	vm_run(vm);

	screen[x][y] = vm->regs[Reg_Out] * 255;
}

VM vm_init(Program prog) {
	VM vm = VM{};

	vm.prog = prog;

	return vm;
}
