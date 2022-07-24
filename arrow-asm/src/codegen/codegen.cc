#include "codegen.hh"

#include <memory>

#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/LegacyPassManager.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Verifier.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/Host.h"
#include "llvm/Support/TargetRegistry.h"
#include "llvm/Support/TargetSelect.h"
#include "llvm/Target/TargetMachine.h"
#include "llvm/Target/TargetOptions.h"

using namespace llvm;
using namespace bytecode;

std::pair<llvm::Value *, llvm::Value *> popTwo(std::vector<llvm::Value *> &stack) {
    llvm::Value *a = *(stack.end() - 2);
    llvm::Value *b = *(stack.end() - 1);
    stack.erase(stack.end() - 2, stack.end());
    return std::make_pair(a, b);
}

llvm::Value *popOne(std::vector<llvm::Value *> &stack) {
    llvm::Value *a = *(stack.end() - 1);
    stack.pop_back();
    return a;
}

class Codegen {
  private:
    std::unique_ptr<LLVMContext> context;
    std::unique_ptr<Module> module;
    std::unique_ptr<IRBuilder<>> builder;
    BytecodeExecutable &bytecode;

    Function *printfFn;
    GlobalValue *longFmtStr;
    GlobalValue *charFmtStr;
    GlobalValue *doubleFmtStr;

    std::vector<Function *> functions;

    void declareFunctions();
    void defineFunctions();
    void emitFunction(Function *f, BytecodeChunk &chunk, Address &address);
    GlobalVariable *createConstantStr(std::string_view str);

  public:
    Codegen(BytecodeExecutable &bytecode);
    void generate();
    void writeObject(std::string_view outputPath);
};

void Codegen::declareFunctions() {
    if (functions.size() > 0)
        return; // functions already declared

    auto voidTy = Type::getVoidTy(*context);
    auto *fnTy = FunctionType::get(voidTy, false);
    for (size_t i = 0; i < bytecode.chunks.size(); i++) {
        auto name = "fn" + std::to_string(i);
        auto *f = Function::Create(fnTy, Function::ExternalLinkage, name, *module);
        functions.push_back(f);
    }
}

void Codegen::defineFunctions() {
    if (bytecode.chunks.size() != functions.size())
        return; // functions not declared yet (TODO: log)

    for (size_t i = 0; i < bytecode.chunks.size(); i++) {
        auto &chunk = bytecode.chunks[i];
        auto &address = bytecode.chunk_locations[i];
        auto *f = functions[i];

        emitFunction(f, chunk, address);
    }
}

void Codegen::emitFunction(Function *f, BytecodeChunk &chunk, Address &address) {
    BasicBlock *bb = BasicBlock::Create(*context, "entry", f);
    builder->SetInsertPoint(bb);

    std::vector<llvm::Value *> stack;

    for (auto &ins : chunk.code) {
        switch (ins.opcode) {
        case kTrap:
            // TODO
            break;
        case kReturn:
            builder->CreateRetVoid();
            break;
        case kBreakpoint:
            // TODO
            break;
        case kPrintLong: {
            std::vector<llvm::Value *> args = {longFmtStr, popOne(stack)};
            builder->CreateCall(printfFn, args);
            break;
        }
        case kPrintChar: {
            std::vector<llvm::Value *> args = {charFmtStr, popOne(stack)};
            builder->CreateCall(printfFn, args);
            break;
        }
        case kNop:
            break;
        case kAddLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateAdd(a, b));
            break;
        }
        case kSubLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateSub(a, b));
            break;
        }
        case kMulLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateMul(a, b));
            break;
        }
        case kIDivLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateSDiv(a, b));
            break;
        }
        case kModuloLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateSRem(a, b));
            break;
        }
        case kLeftShiftLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateShl(a, b));
            break;
        }
        case kRightShiftLogicalLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateLShr(a, b));
            break;
        }
        case kRightShiftArithmeticLong: {
            auto [a, b] = popTwo(stack);
            stack.push_back(builder->CreateAShr(a, b));
            break;
        }
        case kImmByte: {
            auto *ty = Type::getInt64Ty(*context);
            auto *val = ConstantInt::get(ty, (int64_t)ins.param, true);
            stack.push_back(val);
            break;
        }
        case kDup: {
            stack.push_back(stack.back());
            break;
        }
        case kRot2:
            // TODO
            break;
        case kRot3:
            // TODO
            break;
        default:
            // TODO: unimplemented
            break;
        }
    }

    verifyFunction(*f);
}

GlobalVariable *Codegen::createConstantStr(std::string_view str) {
    auto elemTy = Type::getInt8Ty(*context);
    auto arrTy = ArrayType::get(elemTy, str.size() + 1);
    // Const address space, I couldn't find an enum outside of in the specific target headers
    std::vector<Constant *> items;
    for (size_t i = 0; i < str.size(); i++) {
        char c = str.data()[i];
        auto *val = ConstantInt::get(elemTy, c, true);
        items.push_back(val);
    }
    items.push_back(Constant::getNullValue(elemTy));
    auto arr = ConstantArray::get(arrTy, items);
    auto global = new GlobalVariable(*module, arrTy, true, GlobalValue::InternalLinkage, arr, "",
                                     nullptr, GlobalVariable::NotThreadLocal, 4);
    return global;
}

Codegen::Codegen(BytecodeExecutable &bytecode) : bytecode{bytecode} {
    context = std::make_unique<LLVMContext>();
    module = std::make_unique<Module>("beautiful-asm-test", *context);
    builder = std::make_unique<IRBuilder<>>(*context);

    auto *i32 = Type::getInt32Ty(*context);
    auto *ptr = Type::getInt8PtrTy(*context);
    auto *printfTy = FunctionType::get(i32, ptr, true);
    printfFn = Function::Create(printfTy, Function::ExternalLinkage, "printf", *module);
    longFmtStr = createConstantStr("%li");
    charFmtStr = createConstantStr("%c");
    doubleFmtStr = createConstantStr("%f");
}

void Codegen::generate() {
    declareFunctions();
    defineFunctions();
}

void Codegen::writeObject(std::string_view outputPath) {
    InitializeAllTargetInfos();
    InitializeAllTargets();
    InitializeAllTargetMCs();
    InitializeAllAsmPrinters();

    auto targetTriple = sys::getDefaultTargetTriple();
    std::string err;
    auto target = TargetRegistry::lookupTarget(targetTriple, err);

    if (!target) {
        // TODO: log `err`
        return;
    }

    // use generic cpu for now
    auto cpu = "generic";
    auto features = "";
    TargetOptions opt;
    auto rm = Optional<Reloc::Model>();
    auto targetMachine = target->createTargetMachine(targetTriple, cpu, features, opt, rm);

    module->setDataLayout(targetMachine->createDataLayout());
    module->setTargetTriple(targetTriple);

    std::error_code ec;
    raw_fd_ostream dest(outputPath, ec, sys::fs::OF_None);
    if (ec) {
        // TODO: error: could not open file
        return;
    }

    legacy::PassManager pass;
    auto fileType = CGFT_ObjectFile;

    if (targetMachine->addPassesToEmitFile(pass, dest, nullptr, fileType)) {
        // TODO: error: failed to add emit pass
        return;
    }

    pass.run(*module);
    dest.flush();
}

namespace codegen {

void Generate(BytecodeExecutable &bytecode, std::string_view outputPath) {
    Codegen codegen(bytecode);
    codegen.generate();
    codegen.writeObject(outputPath);
}

} // end namespace codegen
