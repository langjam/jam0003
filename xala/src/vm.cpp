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

static inline
void vm_instr_pow(VM *vm) {
	float a = vm_pop(vm);
	float b = vm_pop(vm);

	vm_push(vm, pow(b, a));
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

static inline
void vm_instr_copy(VM *vm, int arg) {
	int base = 0;
	if (vm->csk.calls_len)
		base = vm->csk.calls[vm->csk.calls_len - 1].base;

	vm_push(vm, vm->sk.values[base + arg]);
}

static inline
void vm_instr_sin(VM *vm) {
	vm_push(vm, __builtin_sin(vm_pop(vm)));
}

static inline
void vm_instr_cos(VM *vm) {
	vm_push(vm, __builtin_cos(vm_pop(vm)));
}

static inline
int vm_check_address(VM *vm, int addr) {
	if (addr < 0 || addr >= MEMORY_SIZE) {
		tprintf("SEGFAULT\n");
		return 1;
	}

	return 0;
}

static inline
int vm_memory_load(VM *vm) {
	const float base = vm->regs[Reg_Base];

	CHECKOUT(vm_check_address(vm, base));

	vm_push(vm, vm->mem[(int)base]);

	return 0;
}

static inline
int vm_memory_store(VM *vm) {
	const float base = vm->regs[Reg_Base];

	CHECKOUT(vm_check_address(vm, base));

	vm->mem[(int)base] = vm_pop(vm);

	return 0;
}

int vm_run(VM *vm) {
	uint base = 0;
	uint it = 0;
	while (true) {
		const Instr is = vm->prog.instrs[vm->ip];
		if (it++ > 10000) {
			tprintf("VM OVERLOADED {}!\n", vm->ip);
			return 1;
		}

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
				if (is.argument == Reg_Memory)
					CHECKOUT(vm_memory_load(vm));
				else
					vm_push(vm, vm->regs[is.argument]);
				break;

			case InstrType_Store: {
				if (is.argument == Reg_Memory)
					CHECKOUT(vm_memory_store(vm));
				else
					vm->regs[is.argument] = vm_pop(vm);
			} break;

			case InstrType_SetBase:
				base = vm->sk.values_len-is.argument;
				break;

			case InstrType_Call:
				CHECKOUT(is.argument < 0 || is.argument >= vm->prog.instrs_len);
				CHECKOUT(vm->csk.calls_len >= STACK_MAX - 1);

				vm->csk.calls[vm->csk.calls_len++] = (Call){
					.ret = vm->ip + 1,
					.base = base
				};
				vm->ip = is.argument;
				continue;
				break;

			case InstrType_Ret: {
				if (vm->csk.calls_len == 0) {
					return 0;
				}
			  Call c = vm->csk.calls[--vm->csk.calls_len];
				vm->ip = c.ret;
				vm->sk.values_len = c.base;
				vm_push(vm, vm->regs[Reg_Ret]);
				continue;
			} break;

			case InstrType_Copy:
				vm_instr_copy(vm, is.argument);
				break;

			case InstrType_Sin:
				vm_instr_sin(vm);
				break;

			case InstrType_Cos:
				vm_instr_cos(vm);
				break;

			case InstrType_Pow:
				vm_instr_pow(vm);
				break;

			case InstrType_Ba:
			  vm->ip = is.argument;
			  continue;
				break;

		  case InstrType_Bp:
		  	if (vm_pop(vm) > 0) {
				  vm->ip = is.argument;
		  	} else {
		  		vm->ip++;
		  	}
			  continue;
				break;

		  case InstrType_Bn:
		  	if (vm_pop(vm) < 0)
				  vm->ip = is.argument;
				else
					vm->ip++;
			  continue;
				break;

			case InstrType_Bz:
		  	if (vm_pop(vm) == 0)
				  vm->ip = is.argument;
				else
					vm->ip++;

			  continue;
				break;

			case InstrType_Bnz:
		  	if (vm_pop(vm) != 0)
				  vm->ip = is.argument;
				else
					vm->ip++;



			  continue;
				break;

			case InstrType_Exit:
				return 0;
				break;

			default:
				tprintf("VM {}: unknown instruction\n", vm->ip);
				return 1;
		}
		vm->ip += 1;
		if (vm->sk.values_len < 3) {
			tprintf("VM: STACK UNDEFLOW {} {}\n", vm->sk.values_len, vm->ip);
			return 1;
		}
	}

	return 0;
}

int parity = 0;

int vm_run_scr(VM *vm, u8 screen[256][256]) {
	parity += 1;
	int period = 3;
	int s = parity%period;
	int z = parity/period%period;
	for (int x = z; x < 256; x += period) {
		for (int y = s; y < 256; y += period) {

			vm->ip = vm->prog.start;


			vm->csk.calls_len = 0;
			vm->sk.values_len = 3;
			vm->regs[Reg_X] = x/255.0;
			vm->regs[Reg_Y] = (255-y)/255.0;

			CHECKOUT(vm_run(vm));

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

	vm.sk.values_len = 3;
	vm.prog = prog;

	return vm;
}
