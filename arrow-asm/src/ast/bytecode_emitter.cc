#include "bytecode_emitter.hh"

#include <cassert>
#include <cstddef>
#include <utility>
#include <variant>
#include <array>
#include <optional>
#include <unordered_map>
#include <unordered_set>
#include <list>
#include <vector>

#include "src/ast/ast.hh"
#include "src/bytecode/opcodes.hh"
#include "src/bytecode/bytecode.hh"
#include "spdlog/spdlog.h"

struct FuncInstContext {
  /// @brief Holds T/F for each p register.
  /// If true, is not a valid argument for any instruction.
  std::array<bool, 16> p_usage_map;
  /// @brief The types of the or registers for this function
  std::array<std::optional<ast::RegisterTypeNode>, 64> reg_types;
  /// @brief Parent context (for nested contexts, ex, if/loops)
  FuncInstContext* parent = nullptr;
  private:
  /// @brief The rvalue that is currently on the top of the stack, or nullopt for unknown/nothing/not an rvalue
  std::list<ast::ArgNode> top_list;
  /// @brief Map from immediate to constant index in chunk.
  std::unordered_map<int64_t, int> constants;

  public:
  std::optional<int> has_constant(int64_t val) {
    if (parent) {
      return parent->has_constant(val);
    }
    auto itr = constants.find(val);
    if (itr != constants.end()) {
      return itr->second;
    }
    return std::nullopt;
  }
  void add_constant(int64_t val, int idx) {
    if (parent) {
      parent->add_constant(val, idx);
    } else {
      constants.insert({val, idx});
    }
  }

  void assign_top(ast::ArgNode const& arg) {
    top_list.push_front(arg);
  }
  void pop_top() {
    top_list.pop_front();
  }

  bool same_as_top(ast::ArgNode const& arg) const {
    if (!top_list.empty()) {
      auto const& topref = top_list.front();
      if (std::holds_alternative<ast::ImmediateNode>(topref)
          && std::holds_alternative<ast::ImmediateNode>(arg)) {
        auto const& top_imm = std::get<ast::ImmediateNode>(topref);
        auto const& arg_imm = std::get<ast::ImmediateNode>(arg);
        if (std::holds_alternative<int64_t>(top_imm)
            && std::holds_alternative<int64_t>(arg_imm)) {
          return std::get<int64_t>(top_imm) == std::get<int64_t>(arg_imm);
        } else if (std::holds_alternative<double>(top_imm)
                   && std::holds_alternative<double>(arg_imm)) {
          return std::get<double>(top_imm) == std::get<double>(arg_imm);
        }
        return false;
      } else if (std::holds_alternative<ast::RValueNode>(topref)
                 && std::holds_alternative<ast::RValueNode>(arg)) {
        auto const& top_r = std::get<ast::RValueNode>(topref);
        auto const& arg_r = std::get<ast::RValueNode>(arg);
        return top_r.category == arg_r.category && top_r.register_id == arg_r.register_id;
      }
    }
    return false;
  }
};

void Emit(bytecode::BytecodeChunk& chunk, bytecode::Instruction const& inst) {
  chunk.code.push_back(inst);
}

void EmitNoParam(bytecode::BytecodeChunk& chunk, bytecode::Opcode opcode) {
  Emit(chunk, bytecode::Instruction{
    .opcode = opcode,
    .param = 0
  });
}

int8_t TranslateRegister(ast::RegisterNode const& node) {
  switch (node.category) {
    // TODO: Assert that the register_id is valid!
    case ast::RegisterCategory::Return:
    case ast::RegisterCategory::Param:
      return 48 + node.register_id;
    case ast::RegisterCategory::Local:
      return 32 + node.register_id;
    case ast::RegisterCategory::OutgoingParam:
      return 0 + node.register_id;
    case ast::RegisterCategory::Global:
      // TODO this needs some sort of special handling
    default:
      return node.register_id;
  }
}

/// @brief Returns true if the two ArgNodes are equivalent types.
/// If they are both immediates, they have the same type,
/// or if they are both registers, they hold the same register type.
bool equal_type(FuncInstContext const& ctx, ast::ArgNode const& arg1, ast::ArgNode const& arg2) {
  if (std::holds_alternative<ast::ImmediateNode>(arg1)
      && std::holds_alternative<ast::ImmediateNode>(arg2)) {
    auto const& top_imm = std::get<ast::ImmediateNode>(arg1);
    auto const& arg_imm = std::get<ast::ImmediateNode>(arg2);
    if (std::holds_alternative<int64_t>(top_imm)
        && std::holds_alternative<int64_t>(arg_imm)) {
      return true;
    } else if (std::holds_alternative<double>(top_imm)
                && std::holds_alternative<double>(arg_imm)) {
      return true;
    }
    return false;
  } else if (std::holds_alternative<ast::RValueNode>(arg1)
              && std::holds_alternative<ast::RValueNode>(arg2)) {
    auto const& arg1_r = std::get<ast::RValueNode>(arg1);
    auto const& arg2_r = std::get<ast::RValueNode>(arg2);
    // Register types can't be optionals at this point
    // We must have SOME knowledge of them (since these are RValues).
    // Unless we don't, in which case they are invalid registers.
    auto const& t1 = ctx.reg_types[TranslateRegister(arg1_r)];
    auto const& t2 = ctx.reg_types[TranslateRegister(arg2_r)];
    if (!t1 || !t2) {
      // If we don't know either type, we simply assume we are of correct type
      // and let our runtime figure it out.
      return true;
    }
    return t1.value() == t2.value();
  }
  return false;
}

void EmitArg(bytecode::BytecodeChunk& chunk, FuncInstContext& ctx, ast::ArgNode const& arg) {
  // ASSUME: We should have already checked our arg type to ensure it works with our instruction
  if (ctx.same_as_top(arg)) {
    // We can emit a dup!
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::Opcode::kDup,
      .param = 0
    });
  } else {
    // If we are an imm, emit that value
    if (std::holds_alternative<ast::ImmediateNode>(arg)) {
      auto const& imm = std::get<ast::ImmediateNode>(arg);
      if (std::holds_alternative<int64_t>(imm)) {
        auto val = std::get<int64_t>(imm);
        // Check to see if this is a constant, if it is, grab it
        auto constant_idx_opt = ctx.has_constant(val);
        int constant_idx;
        if (constant_idx_opt) {
          // Found the constant, use the index
          constant_idx = *constant_idx_opt;
        } else {
          constant_idx = chunk.constants.size();
          chunk.constants.push_back(val);
          ctx.add_constant(val, constant_idx);
        }
        // Need to add it as a constant
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kConstant,
          .param = static_cast<int8_t>(constant_idx)
        });
      } else {
        // TODO: ERROR, don't support floating point imms?
      }
    } else if (std::holds_alternative<ast::RValueNode>(arg)) {
      // otherwise, emit that translated register
      // TODO: Need to disallow using a pointer register (sometimes)
      
      Emit(chunk, bytecode::Instruction{
        .opcode = bytecode::Opcode::kLoadAuxiliary,
        .param = TranslateRegister(std::get<ast::RValueNode>(arg))
      });
    }
  }
  // Still add it to top even if it's a duplicate, makes cleanup easier.
  ctx.assign_top(arg);
}

void HandleInstruction(bytecode::BytecodeExecutable& exe, bytecode::BytecodeChunk& chunk, FuncInstContext& ctx, ast::InstructionNode const& inst);

void EmitIf(bytecode::BytecodeExecutable& exe, bytecode::BytecodeChunk& chunk, FuncInstContext& ctx,
            std::vector<std::shared_ptr<ast::InstructionNode>> const& body, std::optional<ast::ArgNode> condition,
            std::vector<int>& lasts, std::vector<FuncInstContext>& contexts) {
  int first;
  // Create context
  contexts.emplace_back();
  auto& nested_ctx = contexts.back();
  nested_ctx.parent = &ctx;
  // Copy our (known) type information to our nested context.
  std::copy(ctx.reg_types.begin(), ctx.reg_types.end(), nested_ctx.reg_types.begin());
  if (condition) {
    // First, we handle our if condition
    EmitArg(chunk, ctx, *condition);
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::kLogicalNeg,
      .param = 0
    });
    // We need to negate so that TestAndJump jumps us if we do NOT match
    first = chunk.code.size();
    ctx.pop_top();
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::Opcode::kTestAndJump,
      .param = 0 // PLACEHOLDER
    });
  }
  
  // Now we handle the if body
  for (auto const& i : body) {
    HandleInstruction(exe, chunk, nested_ctx, *i);
  }
  lasts.push_back(chunk.code.size());
  if (condition) {
    // Only need to emit a final jump if we aren't an else block
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::Opcode::kJump,
      .param = 0 // PLACEHOLDER
    });
  }
  if (condition) {
    // We can go back now to our first and modify the param
    chunk.code[first].param = static_cast<int8_t>(chunk.code.size() - first);
  }
}

void EmitWhile(bytecode::BytecodeExecutable& exe, bytecode::BytecodeChunk& chunk, FuncInstContext& ctx,
            std::vector<std::shared_ptr<ast::InstructionNode>> const& body, ast::ArgNode condition,
            FuncInstContext& nested_ctx) {
  nested_ctx.parent = &ctx;
  // Copy our (known) type information to our nested context.
  std::copy(ctx.reg_types.begin(), ctx.reg_types.end(), nested_ctx.reg_types.begin());

  // First, we handle our loop condition
  int loop_header = chunk.code.size();
  EmitArg(chunk, ctx, condition);
  Emit(chunk, bytecode::Instruction{
    .opcode = bytecode::kLogicalNeg,
    .param = 0
  });
  // We need to negate so that TestAndJump jumps us if we do NOT match
  int conditional_jump = chunk.code.size();
  ctx.pop_top();
  Emit(chunk, bytecode::Instruction{
    .opcode = bytecode::Opcode::kTestAndJump,
    .param = 0 // PLACEHOLDER
  });
  
  // Now we handle the loop body
  for (auto const& i : body) {
    HandleInstruction(exe, chunk, nested_ctx, *i);
  }

  int header_offset = loop_header - (int) chunk.code.size();
  // jump back to loop header
  Emit(chunk, bytecode::Instruction{
    .opcode = bytecode::Opcode::kJump,
    .param = (int8_t) header_offset
  });

  // We can go back now to our first and modify the param
  chunk.code[conditional_jump].param = static_cast<int8_t>(chunk.code.size() - conditional_jump);
}

std::string symb_name(ast::ObjectTypeNode const& o) {
  if (std::holds_alternative<ast::LongNode>(o)) {
    return "long";
  } else if (std::holds_alternative<ast::DoubleNode>(o)) {
    return "double";
  } else if (std::holds_alternative<ast::PtrNode>(o)) {
    auto const& ptr = std::get<ast::PtrNode>(o);
    return "ptr<" + symb_name(*ptr.element_type) + ">";
  } else {
    return "";
  }
}

// TODO: Return bool?
void HandleInstruction(bytecode::BytecodeExecutable& exe, bytecode::BytecodeChunk& chunk, FuncInstContext& ctx, ast::InstructionNode const& inst) {
  auto const& instr = inst;
  // ArrowInstNode, NoArgNode, NoRetNode, BinaryNode, MemoryNode, IfNode, WhileNode
  if (std::holds_alternative<ast::IfNode>(instr)) {
    auto const& ifstmt = std::get<ast::IfNode>(instr);
    // If statements (and all nested statements) are kind of interesting
    // We keep our context, but we need a new one (so we don't taint our existing one)
    // per if/elif/else branch.
    // Then, at the end, we can union all of these contexts together
    // to continue to get the best possible type checking we can
    std::vector<FuncInstContext> contexts;
    int blocks = 1 + ifstmt.elifs.size() + (ifstmt.else_node ? 1 : 0);
    contexts.reserve(blocks);
    // lasts is a vector of pointers to our last instruction in each if
    // essentially, the 'jump to end' logic
    std::vector<int> lasts;
    lasts.reserve(blocks);

    EmitIf(exe, chunk, ctx, ifstmt.body, ifstmt.condition, lasts, contexts);
    // For each elif, do the same thing:
    for (auto const& elif : ifstmt.elifs) {
      EmitIf(exe, chunk, ctx, elif.body, elif.condition, lasts, contexts);
    }

    if (ifstmt.else_node) {
      EmitIf(exe, chunk, ctx, ifstmt.else_node->body, std::nullopt, lasts, contexts);
    }
    // Ensure we have a landing pad for our nested blocks to land at
    auto landing_pad_idx = chunk.code.size();
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::Opcode::kNop,
      .param = 0
    });
    // Now, walk our lasts and fix them up
    for (auto idx : lasts) {
      chunk.code[idx].param = static_cast<int8_t>(landing_pad_idx - idx); 
    }
    // TODO: Walk our types and apply a union to our top level context
  } if (std::holds_alternative<ast::WhileNode>(instr)) {
    auto const& while_loop = std::get<ast::WhileNode>(instr);

    FuncInstContext nested_ctx;
    EmitWhile(exe, chunk, ctx, while_loop.body, while_loop.condition, nested_ctx);

    // Ensure we have a landing pad when condition fails
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::Opcode::kNop,
      .param = 0
    });
    // TODO: Walk our types and apply a union to our top level context
  } else if (std::holds_alternative<ast::NoArgNode>(instr)) {
    auto& node = std::get<ast::NoArgNode>(instr);
    switch (node.op) {
      case ast::NoArgOperator::kTrap: {
        // Easy.
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kTrap,
          .param = 0
        });
        break;
      }
      case ast::NoArgOperator::kBreak:
      case ast::NoArgOperator::kContinue:

      case ast::NoArgOperator::kRet: {
        // To return, we need to have (on the top of our stack)
        // all of our return values. They would come from all of our prior p and rr writes
        // Alternative, we push all of our return writes to the VERY bottom of our stack
        // and then we pop all of our parameters here before we ret
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kReturn,
          .param = 0
        });
        break;
      }
    }
  } else if (std::holds_alternative<ast::NoRetNode>(instr)) {
    auto& node = std::get<ast::NoRetNode>(instr);
    switch (node.op) {
      case ast::NoRetOperator::kPrint: {
        // We are to print. or0 is our parameter
        // We should ? already have or0 on the top of our stack
        // So just perform a print (ideally we use or0's type)
        auto const& arg = node.arg;
        EmitArg(chunk, ctx, arg);
        // TODO: We should check our type here to see what we need to emit
        // Pop our argument off the top
        ctx.pop_top();
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kPrintLong,
          .param = 0
        });
        break;
      }
      case ast::NoRetOperator::kPrintChar: {
        auto const& arg = node.arg;
        EmitArg(chunk, ctx, arg);
        // TODO: type checking
        ctx.pop_top();
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kPrintChar,
          .param = 0
        });
        break;
      }
      case ast::NoRetOperator::kExit: {
        auto const& arg = node.arg;
        EmitArg(chunk, ctx, arg);
        // TODO: type checking
        ctx.pop_top();
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kExit,
          .param = 0
        });
        break;
      }
      default:
        // TODO: ERROR
        break;
    }
  } else if (std::holds_alternative<ast::UnaryNode>(instr)) {
    auto const& node = std::get<ast::UnaryNode>(instr);
    auto const& arg = node.rhs;
    EmitArg(chunk, ctx, arg);
    switch (node.op) {
      case ast::UnaryOperator::kANeg: 
        EmitNoParam(chunk, bytecode::Opcode::kArithmeticNeg);
        break;
      case ast::UnaryOperator::kBNeg: 
        EmitNoParam(chunk, bytecode::Opcode::kBinaryNeg);
        break;
      case ast::UnaryOperator::kLNeg: 
        EmitNoParam(chunk, bytecode::Opcode::kLogicalNeg);
        break;
      default:
        // TODO: ERROR (Unsupported unary inst)
        break;
    }
    // Pop both arguments from our 'top' stack
    ctx.pop_top();
    // Emit the store
    auto dst = TranslateRegister(node.lhs);
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::Opcode::kStoreAuxiliary,
      .param = dst
    });
    // TODO: Properly handle type assignment
    ctx.reg_types[dst] = ast::LongNode();
  } else if (std::holds_alternative<ast::BinaryNode>(instr)) {
    auto const& node = std::get<ast::BinaryNode>(instr);
    auto const& arg1 = node.arg1;
    auto const& arg2 = node.arg2;
    // We want to make sure both of our arguments are of an equivalent type
    if (!equal_type(ctx, arg1, arg2)) {
      // TODO: ERROR (Do not have equivalent type for arguments!)
    }
    EmitArg(chunk, ctx, arg1);
    EmitArg(chunk, ctx, arg2);
    switch (node.op) {
      case ast::BinaryOperator::kAdd: 
        EmitNoParam(chunk, bytecode::Opcode::kAddLong);
        break;
      case ast::BinaryOperator::kSub: 
        EmitNoParam(chunk, bytecode::Opcode::kSubLong);
        break;
      case ast::BinaryOperator::kMul: 
        EmitNoParam(chunk, bytecode::Opcode::kMulLong);
        break;
      case ast::BinaryOperator::kDiv: 
        EmitNoParam(chunk, bytecode::Opcode::kIDivLong);
        break;
      case ast::BinaryOperator::kMod: 
        EmitNoParam(chunk, bytecode::Opcode::kModuloLong);
        break;
      case ast::BinaryOperator::kSll: 
        EmitNoParam(chunk, bytecode::Opcode::kLeftShiftLong);
        break;
      case ast::BinaryOperator::kSrl: 
        EmitNoParam(chunk, bytecode::Opcode::kRightShiftLogicalLong);
        break;
      case ast::BinaryOperator::kSra: 
        EmitNoParam(chunk, bytecode::Opcode::kRightShiftArithmeticLong);
        break;
      case ast::BinaryOperator::kAnd: 
        EmitNoParam(chunk, bytecode::Opcode::kLogicalAndLong);
        break;
      case ast::BinaryOperator::kOr: 
        EmitNoParam(chunk, bytecode::Opcode::kLogicalOrLong);
        break;
      default:
        // TODO: ERROR (Unsupported binary inst)
        break;
    }
    // Pop both arguments from our 'top' stack
    ctx.pop_top();
    ctx.pop_top();
    // Emit the store
    auto dst = TranslateRegister(node.lhs);
    Emit(chunk, bytecode::Instruction{
      .opcode = bytecode::Opcode::kStoreAuxiliary,
      .param = dst
    });
    // TODO: Properly handle type assignment
    ctx.reg_types[dst] = ast::LongNode();
  } else if (std::holds_alternative<ast::CallNode>(instr)) {
    auto const& call = std::get<ast::CallNode>(instr);
    auto itr = exe.symbol_table.find(call.id.id);
    if (itr == exe.symbol_table.end()) {
      // TODO: ERROR (identifier could not be found)
    } else {
      if (std::holds_alternative<bytecode::ChunkId>(itr->second)) {
        auto chunkId = std::get<bytecode::ChunkId>(itr->second).idx;
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kCall,
          .param = static_cast<int8_t>(chunkId)
        });
      } else {
        // TODO: ERROR (Identifier is not a function)
      }
    }
  } else if (std::holds_alternative<ast::ArrowInstNode>(instr)) {
    auto const& arrow = std::get<ast::ArrowInstNode>(instr);
    if (std::holds_alternative<ast::RValueNode>(arrow.rhs)) {
      auto const& rhs = std::get<ast::RValueNode>(arrow.rhs);
      Emit(chunk, bytecode::Instruction{
        .opcode = bytecode::Opcode::kMoveAuxiliary,
        .param = TranslateRegister(rhs)
      });
    } else if (std::holds_alternative<ast::MemberNode>(arrow.rhs)) {
      auto const& rhs = std::get<ast::MemberNode>(arrow.rhs);
      auto symbol = rhs.type.id + ':' + rhs.field.id;
      auto itr = exe.symbol_table.find(symbol);
      if (itr == exe.symbol_table.end()) {
        // TODO: ERROR (field symbol not found)
      } else {
        if (std::holds_alternative<bytecode::FieldId>(itr->second)) {
          auto fid = std::get<bytecode::FieldId>(itr->second).idx;
          // Load the pointer on stack
          Emit(chunk, bytecode::Instruction{
            .opcode = bytecode::Opcode::kLoadAuxiliary,
            .param = TranslateRegister(rhs.obj)
          });
          // Move the field to stack
          Emit(chunk, bytecode::Instruction{
            .opcode = bytecode::Opcode::kMoveOutObjectField,
            .param = static_cast<int8_t>(fid)
          });
        } else {
          // TODO: ERROR (symbol is not a field)
        }
      }
    } else if (std::holds_alternative<ast::MakeNode>(arrow.rhs)) {
      auto const& make = std::get<ast::MakeNode>(arrow.rhs);
      auto const& type = make.type;
      if (std::holds_alternative<ast::IdNode>(type)) {
        auto const& id = std::get<ast::IdNode>(type);
        auto itr = exe.symbol_table.find(id.id);
        if (itr == exe.symbol_table.end()) {
          // TODO: ERROR (type symbol not found)
        } else {
          if (std::holds_alternative<bytecode::ClassId>(itr->second)) {
            auto cid = std::get<bytecode::ClassId>(itr->second).idx;
            // Flow:
            auto const& klass = exe.classes[cid];
            auto sz = (klass.fields.size() + 1) * sizeof(bytecode::Value);
            // 1. Allocate
            // TODO: Emit this only when safe to do so (ex, sz < 128)
            Emit(chunk, bytecode::Instruction{
              .opcode = bytecode::Opcode::kAllocateImm,
              .param = static_cast<int8_t>(sz)
            });
            Emit(chunk, bytecode::Instruction{
              .opcode = bytecode::Opcode::kStoreAuxiliary,
              .param = TranslateRegister(ast::RegisterNode{
                .category = ast::RegisterCategory::OutgoingParam,
                .register_id = 0
              })
            });
            // 2. Call constructor
            Emit(chunk, bytecode::Instruction{
              .opcode = bytecode::Opcode::kCall,
              .param = static_cast<int8_t>(klass.ctor_chunk.idx)
            });
            // 3. Have pointer on stack
            Emit(chunk, bytecode::Instruction{
              .opcode = bytecode::Opcode::kMoveAuxiliary,
              .param = TranslateRegister(ast::RegisterNode{
                .category = ast::RegisterCategory::OutgoingParam,
                .register_id = 0
              })
            });
          } else {
            // TODO: ERROR (symbol is not a type)
          }
        }
      } else {
        std::string symb = symb_name(type);

        // TODO: Handle boxing types.
        // This is actually pretty complex because a boxed type should have a
        // class index that is not in use.
        // We can do this by just making it on the spot--
        // We can also abuse the symbol table a little and put these boxed
        // types in there as well
        auto itr = exe.symbol_table.find(symb);
        int cid;
        if (itr != exe.symbol_table.end()) {
          // Use the existing one
          if (std::holds_alternative<bytecode::ClassId>(itr->second)) {
            cid = std::get<bytecode::ClassId>(itr->second).idx;
          } else {
            // TODO: ERROR (symbol table does not have this primitive! Name collision?)
          }
        } else {
          // We should make a new box class record
          cid = exe.classes.size();
          exe.symbol_table.insert({symb, bytecode::ClassId{ .idx = cid }});
          bytecode::BytecodeExecutable::ClassDataRecord klass{};
          klass.ctor_chunk = bytecode::ChunkId { .idx = 0 };
          klass.dtor_chunk = bytecode::ChunkId { .idx = 0 };
          klass.name = symb;
          if (std::holds_alternative<ast::LongNode>(type)) {
            klass.fields.push_back(bytecode::BytecodeExecutable::ValueType::kLong);
          } else if (std::holds_alternative<ast::DoubleNode>(type)) {
            klass.fields.push_back(bytecode::BytecodeExecutable::ValueType::kDouble);
          } else if (std::holds_alternative<ast::PtrNode>(type)) {
            klass.fields.push_back(bytecode::BytecodeExecutable::ValueType::kDataPtr);
          } else {
            // TODO: ERROR (impossible?)
          }
          exe.classes.push_back(klass);
        }
        // Size of the type is 2 fields (implicit + extra)
        int8_t sz = 8 * 2;
        // Allocate
        Emit(chunk, bytecode::Instruction{
          .opcode = bytecode::Opcode::kAllocateImm,
          .param = sz
        });
        // Pointer is now on stack, done!
        // Note: The (implicit) dtor for a boxed pointer does not actually destroy
        // this is a potential problem.
        // TODO: See above
      }
    } else {
      // TODO: ERROR (Unsupported arrow rhs)
    }
    // Any time we move like this, we need to make sure that our lhs
    // needs to be destroyed, if necessary.
    // We will basically need to emit some code that would perform this check
    if (std::holds_alternative<ast::LValueNode>(arrow.lhs)) {
      // Here, we will call Destroy to ensure that the lhs is properly destroyed
      // This involves us loading it first (more specifically, MOVING it to stack)
      auto const& lhs = std::get<ast::LValueNode>(arrow.lhs);

      Emit(chunk, bytecode::Instruction{
        .opcode = bytecode::Opcode::kMoveAuxiliary,
        .param = TranslateRegister(lhs)
      });
      Emit(chunk, bytecode::Instruction{
        .opcode = bytecode::Opcode::kDestroy,
        .param = 0
      });
      Emit(chunk, bytecode::Instruction{
        .opcode = bytecode::Opcode::kStoreAuxiliary,
        .param = TranslateRegister(lhs)
      });
    } else if (std::holds_alternative<ast::MemberNode>(arrow.lhs)) {
      auto const& lhs = std::get<ast::MemberNode>(arrow.lhs);

      auto symbol = lhs.type.id + ':' + lhs.field.id;
      auto itr = exe.symbol_table.find(symbol);
      if (itr == exe.symbol_table.end()) {
        // TODO: ERROR (field symbol not found)
      } else {
        if (std::holds_alternative<bytecode::FieldId>(itr->second)) {
          auto fid = std::get<bytecode::FieldId>(itr->second).idx;
          // Load the pointer on stack
          Emit(chunk, bytecode::Instruction{
            .opcode = bytecode::Opcode::kLoadAuxiliary,
            .param = TranslateRegister(lhs.obj)
          });
          Emit(chunk, bytecode::Instruction{
            .opcode = bytecode::Opcode::kStoreObjectField,
            .param = static_cast<int8_t>(fid)
          });
        } else {
          // TODO: ERROR (field identifier is not a field)
        }
      }
    }
  } else {
    // TODO: ERROR (Unsupported instruction)
  }
}

static std::unordered_set<std::string> primitives{
  "long",
  "double",
  "ptr"
};

bytecode::BytecodeExecutable ast::LowerAst(const ProgramNode& ast) {
  bytecode::BytecodeExecutable exe{};
  // Set up empty return chunk
  exe.chunks.push_back(bytecode::BytecodeChunk{
    .code{{bytecode::Instruction{
      .opcode = bytecode::Opcode::kReturn,
      .param = 0
    }}}
  });
  exe.chunk_locations.push_back(0);
  // First pass, add records
  int chunkId = 1;
  int typeId = 0;
  std::unordered_map<std::string, std::pair<int, int>> type_method_map{};
  for (auto const& stmt : ast.statements) {
    if (std::holds_alternative<ast::FunctionNode>(*stmt)) {
      auto func = std::get<ast::FunctionNode>(*stmt);
      // Functions are 1-to-1 to chunks
      exe.chunk_locations.push_back(chunkId);
      if (primitives.find(func.id.id) != primitives.end()) {
        // TODO: ERROR (cannot redefine primitive types!)
      }
      if (exe.symbol_table.find(func.id.id) != exe.symbol_table.end()) {
        // TODO: ERROR (Duplicate function definition/clash!)
      }
      exe.symbol_table.insert({func.id.id, bytecode::ChunkId { .idx = chunkId++ }});
      exe.chunks.push_back(bytecode::BytecodeChunk{});
    } else if (std::holds_alternative<ast::TypeNode>(*stmt)) {
      auto type = std::get<ast::TypeNode>(*stmt);
      int cidx = 0, didx = 0;
      if (type.ctor) {
        cidx = chunkId;
        exe.chunk_locations.push_back(chunkId);
        exe.chunks.push_back(bytecode::BytecodeChunk{});
      }
      if (type.dtor) {
        didx = chunkId;
        exe.chunk_locations.push_back(chunkId);
        exe.chunks.push_back(bytecode::BytecodeChunk{});
      }
      // First field starts at 1, there's an implicit 0th field.
      // BUT, that's taken care of the internals
      int fOffset = 0;
      bytecode::BytecodeExecutable::ClassDataRecord record{
        .name = type.id.id,
        .ctor_chunk = bytecode::ChunkId { .idx = cidx },
        .dtor_chunk = bytecode::ChunkId { .idx = didx },
      };

      for (auto const& f : type.fields) {
        // Add field to symbol table as Type:fid
        exe.symbol_table.insert({type.id.id + ':' + f->id.id, bytecode::FieldId { fOffset++ }});
        if (std::holds_alternative<LongNode>(f->type)) {
          record.fields.push_back(bytecode::BytecodeExecutable::ValueType::kLong);
        } else if (std::holds_alternative<DoubleNode>(f->type)) {
          record.fields.push_back(bytecode::BytecodeExecutable::ValueType::kDouble);
        } else if (std::holds_alternative<PtrNode>(f->type)) {
          record.fields.push_back(bytecode::BytecodeExecutable::ValueType::kDataPtr);
        } else {
          // TODO: ERROR (unknown field type)
        }
      }
      if (primitives.find(type.id.id) != primitives.end()) {
        // TODO: ERROR (cannot redefine primitive types!)
      }
      if (type_method_map.find(type.id.id) != type_method_map.end()) {
        // TODO: ERROR (Duplicate type name!)
      }
      type_method_map.insert({type.id.id, {cidx, didx}});
      if (exe.symbol_table.find(type.id.id) != exe.symbol_table.end()) {
        // TODO: ERROR (Type/method name clash!)
      }
      exe.symbol_table.insert({type.id.id, bytecode::ClassId { .idx = typeId++ }});
      exe.classes.push_back(record);
    }
  }
  for (auto& stmt : ast.statements) {
    if (std::holds_alternative<ast::FunctionNode>(*stmt)) {
      auto func = std::get<ast::FunctionNode>(*stmt);
      // Functions are 1-to-1 of blocks
      auto const& symbol = func.id;
      auto& chunk = exe.chunks[std::get<bytecode::ChunkId>(exe.symbol_table[symbol.id]).idx];
      FuncInstContext ctx{};
      for (int i = 0; i < func.params.size(); ++i) {
        ctx.reg_types[TranslateRegister(ast::RegisterNode{
          .category = ast::RegisterCategory::Param,
          .register_id = static_cast<uint8_t>(i)
        })] = *func.params[i];
      }
      for (auto& inst : func.body) {
        HandleInstruction(exe, chunk, ctx, *inst);
      }
    } else if (std::holds_alternative<ast::TypeNode>(*stmt)) {
      auto type = std::get<ast::TypeNode>(*stmt);
      auto [ctor, dtor] = type_method_map[type.id.id];

      // Handle ctor
      if (type.ctor) {
        auto& chunk = exe.chunks[ctor];
        auto const& func = *type.ctor;
        FuncInstContext ctx{};
        for (int i = 0; i < func.params.size(); ++i) {
          ctx.reg_types[TranslateRegister(ast::RegisterNode{
            .category = ast::RegisterCategory::Param,
            .register_id = static_cast<uint8_t>(i + 1)
          })] = *func.params[i];
        }
        ctx.reg_types[TranslateRegister(ast::RegisterNode{
          .category = ast::RegisterCategory::Param,
          .register_id = static_cast<uint8_t>(0)
        })] = ast::PtrNode{
          .element_type = std::make_shared<ast::ObjectTypeNode>(type.id)
        };
        for (auto& inst : func.body) {
          HandleInstruction(exe, chunk, ctx, *inst);
        }
      }
      // Handle dtor
      if (type.dtor) {
        auto& chunk = exe.chunks[dtor];
        auto const& func = *type.dtor;
        FuncInstContext ctx{};
        ctx.reg_types[TranslateRegister(ast::RegisterNode{
          .category = ast::RegisterCategory::Param,
          .register_id = static_cast<int8_t>(0)
        })] = ast::PtrNode{
          .element_type = std::make_shared<ast::ObjectTypeNode>(type.id)
        };
        for (auto& inst : func.body) {
          HandleInstruction(exe, chunk, ctx, *inst);
        }
      }
    }
  }
  return exe;
}
