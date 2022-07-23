#include "debug.h"
#include "common.h"
#include "vm.h"
#include "vmintrin.h"
#include "wasm.h"

static
int vm_pop(VM *vm, float *o) {
	if (vm->sk.values_len <= 0) {
		tprintf("VM: stack underflow\n");
		CHECKOUT(1);
	}
	*o = vm->sk.values[--vm->sk.values_len];

	return 0;
}

static
int vm_push(VM *vm, float v) {
	if (vm->sk.values_len >= STACK_MAX - 1) {
		tprintf("VM: stack overflow\n");
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
	
	return vm_push(vm, b * a);
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
		tprintf("VM: error: division by zero\n");
		CHECKOUT(1)
	}
	
	return vm_push(vm, b / a);
}

float fmod(float x, float y) {
  return x - int(x / y) * y;
}

static
int vm_instr_mod(VM *vm) {
	float a, b;
	CHECKOUT(vm_pop(vm, &a) || vm_pop(vm, &b));

	if (a == 0) {
		tprintf("VM: error: division by zero\n");
		CHECKOUT(1);
	}
	
	return vm_push(vm, fmod(b, a));
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
				CHECKOUT(vm_pop(vm, &_));
			break;

		case InstrType_Imm:
			CHECKOUT(vm_push(vm, *(float*)&arg));
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
			tprintf("VM: unknown instruction\n");
			return 1;
		}
		vm->ip += 1;
	}

	return 0;
}

int vm_run_for_pixel(VM *vm, u8 screen[256][256], sint x, sint y) {
	vm->ip = 0;
	vm->csk.return_addrs_len = 0;
	vm->sk.values_len = 0;
	vm->regs[Reg_X] = x/255.0;
	vm->regs[Reg_Y] = (255-y)/255.0;

	CHECKOUT(vm_run(vm));

	if (vm->regs[Reg_Out] > 1.0) {
		vm->regs[Reg_Out] = 1.0;
	}

	screen[y][x] = vm->regs[Reg_Out] * 255;
	return 0;
}

VM vm_init(Program prog) {
	VM vm = VM{};

	vm.prog = prog;

	return vm;
}
