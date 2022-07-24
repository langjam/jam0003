#include "debug.h"
#include "common.h"
#include "vm.h"
#include "vmintrin.h"
#include "wasm.h"

static inline
float vm_pop(VM *vm) {
	return vm->sk.values[--vm->sk.values_len];
}

static inline
void vm_push(VM *vm, float v) {
	vm->sk.values[vm->sk.values_len++] = v;
}

static inline
void vm_instr_add(VM *vm) {
	float a, b;
	a = vm_pop(vm);
	b = vm_pop(vm);
	
	vm_push(vm, b + a);
}

static inline
void vm_instr_mul(VM *vm) {
	float a, b;
	a = vm_pop(vm);
	b = vm_pop(vm);
	
	vm_push(vm, b * a);
}

static inline
void vm_instr_sub(VM *vm) {
	float a, b;
	a = vm_pop(vm);
	b = vm_pop(vm);
	
	vm_push(vm, b - a);
}

static inline
void vm_instr_div(VM *vm) {
	float a, b;
	a = vm_pop(vm);
	b = vm_pop(vm);

	vm_push(vm, b / a);
}

float fmod(float x, float y) {
  return x - int(x / y) * y;
}

static inline
void vm_instr_mod(VM *vm) {
	float a, b;
	a = vm_pop(vm);
	b = vm_pop(vm);
	
	vm_push(vm, fmod(b, a));
}

int vm_run(VM *vm) {
	while (true) {
		const Instr is = vm->prog.instrs[vm->ip];

		switch (is.type) {
			case InstrType_Add:
				vm_instr_add(vm);
				break;
			case InstrType_Mul:
				vm_instr_mul(vm);
				break;
			case InstrType_Sub:
				vm_instr_sub(vm);
				break;
			case InstrType_Div:
				vm_instr_div(vm);
				break;
			case InstrType_Mod:
				vm_instr_mod(vm);
				break;

			case InstrType_Pop:
				vm->sk.values_len -= is.argument;
				break;

			case InstrType_Imm:
				vm_push(vm, *(float*)&is.argument);
				break;

			case InstrType_Load:
				vm_push(vm, vm->regs[is.argument]);
				break;

			case InstrType_Store:
				vm->regs[is.argument] = vm_pop(vm);
				break;

			case InstrType_Exit:
				return 0;
				break;

			default:
				tprintf("VM: unknown instruction\n");
				return 1;
		}
		vm->ip += 1;
		//if (vm->sk.values_len < 3) {
		//	return 1;
		//}
	}

	return 0;
}

int vm_run_scr(VM *vm, u8 screen[256][256]) {
	for (int x = 0; x < 256; ++x) {
		for (int y = 0; y < 256; ++y) {
			vm->ip = 0;
			vm->csk.return_addrs_len = 0;
			vm->sk.values_len = 3;
			vm->regs[Reg_X] = x/255.0;
			vm->regs[Reg_Y] = (255-y)/255.0;

			vm_run(vm);

			if (vm->regs[Reg_Out] > 1.0) {
				vm->regs[Reg_Out] = 1.0;
			}

			screen[y][x] = vm->regs[Reg_Out] * 255;
		}
	}
	return 0;
}

VM vm_init(Program prog) {
	VM vm = VM{};


	vm.prog = prog;

	return vm;
}
