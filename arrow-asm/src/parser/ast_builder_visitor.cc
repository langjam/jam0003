#include "src/parser/ast_builder_visitor.hh"
#include "src/parser/AasmBaseVisitor.h"

#include <iostream>
#include <memory>
#include <string>

namespace parser {

std::any ASTBuilderVisitor::visitProgram(AasmParser::ProgramContext *ctx) {
  auto node = ast::ProgramNode();
  for (auto statement : ctx->statement()) {
    node.statements.push_back(std::make_shared<ast::StatementNode>(
        std::any_cast<ast::StatementNode>(visitStatement(statement))));
  }
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any ASTBuilderVisitor::visitFunction_definition(
    AasmParser::Function_definitionContext *ctx) {
  auto node = ast::FunctionNode();
  node.id = ast::IdNode{.id{ctx->ID()->getText()}};
  if (ctx->parameter_list()) {
    node.params =
        std::any_cast<std::vector<std::shared_ptr<ast::RegisterTypeNode>>>(
            visitParameter_list(ctx->parameter_list()));
  }
  if (ctx->instructions()) {
    node.body =
        std::any_cast<std::vector<std::shared_ptr<ast::InstructionNode>>>(
            visitInstructions(ctx->instructions()));
  }
  node.sourcePos = ctx->ID()->getSourceInterval().toString();
  return ast::StatementNode(node);
}

std::any
ASTBuilderVisitor::visitParameter_list(AasmParser::Parameter_listContext *ctx) {
  std::vector<std::shared_ptr<ast::RegisterTypeNode>> param_list;
  for (auto register_type : ctx->register_type()) {
    auto param =
        std::any_cast<ast::RegisterTypeNode>(visitRegister_type(register_type));
    param.sourcePos = register_type->getSourceInterval().toString();
    param_list.push_back(std::make_shared<ast::RegisterTypeNode>(param));
  }
  return param_list;
}

std::any ASTBuilderVisitor::visitType_definition(
    AasmParser::Type_definitionContext *ctx) {
  ast::TypeNode node;
  for (auto ctor : ctx->constructor()) {
    node.ctor = ast::CtorNode{
        {.sourcePos{ctor->getSourceInterval().toString()}},
        .params{(ctor->parameter_list())
                    ? std::any_cast<
                          std::vector<std::shared_ptr<ast::RegisterTypeNode>>>(
                          visitParameter_list(ctor->parameter_list()))
                    : std::vector<std::shared_ptr<ast::RegisterTypeNode>>()},
        .body{std::any_cast<std::vector<std::shared_ptr<ast::InstructionNode>>>(
            visitInstructions(ctor->instructions()))},
    };
  }

  for (auto dtor : ctx->destructor()) {
    node.dtor = ast::DtorNode{
        {.sourcePos{dtor->getSourceInterval().toString()}},
        .body{std::any_cast<std::vector<std::shared_ptr<ast::InstructionNode>>>(
            visitInstructions(dtor->instructions()))}};
  }
  for (auto field : ctx->field()) {
    node.fields.push_back(std::make_shared<ast::FieldNode>(
        std::any_cast<ast::FieldNode>(visitField(field))));
  }
  node.sourcePos = ctx->getSourceInterval().toString();
  return ast::StatementNode(node);
}

std::any
ASTBuilderVisitor::visitInstructions(AasmParser::InstructionsContext *ctx) {
  std::vector<std::shared_ptr<ast::InstructionNode>> instructions;
  for (auto instruction : ctx->instruction()) {
    instructions.push_back(std::any_cast<std::shared_ptr<ast::InstructionNode>>(
        visitInstruction(instruction)));
  }
  return instructions;
}

std::any
ASTBuilderVisitor::visitInstruction(AasmParser::InstructionContext *ctx) {
  if (ctx->unary_operator_instruction())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::UnaryNode>(visitChildren(ctx)));
  if (ctx->binary_operator_instruction())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::BinaryNode>(visitChildren(ctx)));
  if (ctx->call_instruction())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::CallNode>(visitChildren(ctx)));
  if (ctx->print_instruction() || ctx->printch_instruction() ||
      ctx->exit_instruction())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::NoRetNode>(visitChildren(ctx)));
  if (ctx->memory_instruction())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::MemoryNode>(visitChildren(ctx)));
  if (ctx->arrow_instruction())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::ArrowInstNode>(visitChildren(ctx)));
  if (ctx->no_arg_instruction())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::NoArgNode>(visitChildren(ctx)));
  if (ctx->if_statement())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::IfNode>(visitChildren(ctx)));
  if (ctx->while_loop())
    return std::make_shared<ast::InstructionNode>(
        std::any_cast<ast::WhileNode>(visitChildren(ctx)));
  return {};
}

std::any ASTBuilderVisitor::visitNo_arg_instruction(
    AasmParser::No_arg_instructionContext *ctx) {
  auto text = ctx->getText();
  if (text == "trap")
    return ast::NoArgNode{{.sourcePos = ctx->getSourceInterval().toString()},
                          .op = ast::NoArgOperator::kTrap};
  else if (text == "ret")
    return ast::NoArgNode{{.sourcePos = ctx->getSourceInterval().toString()},
                          .op = ast::NoArgOperator::kRet};
  else if (text == "break")
    return ast::NoArgNode{{.sourcePos = ctx->getSourceInterval().toString()},
                          .op = ast::NoArgOperator::kBreak};
  else if (text == "continue")
    return ast::NoArgNode{{.sourcePos = ctx->getSourceInterval().toString()},
                          .op = ast::NoArgOperator::kContinue};
  return {};
}

std::any ASTBuilderVisitor::visitArrow_instruction(
    AasmParser::Arrow_instructionContext *ctx) {
  ast::ArrowInstNode node;
  node.lhs = std::any_cast<ast::ArrowLhsNode>(visitArrow_lhs(ctx->arrow_lhs()));
  node.rhs = std::any_cast<ast::ArrowRhsNode>(visitArrow_rhs(ctx->arrow_rhs()));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any ASTBuilderVisitor::visitArrow_lhs(AasmParser::Arrow_lhsContext *ctx) {
  if (ctx->any_field()) {
    auto node =
        ast::ArrowLhsNode{std::any_cast<ast::MemberNode>(visitChildren(ctx))};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  if (ctx->any_lvalue()) {
    auto node =
        ast::ArrowLhsNode{std::any_cast<ast::LValueNode>(visitChildren(ctx))};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  return {};
}

std::any ASTBuilderVisitor::visitArrow_rhs(AasmParser::Arrow_rhsContext *ctx) {
  if (ctx->any_field()) {
    auto node =
        ast::ArrowRhsNode{std::any_cast<ast::MemberNode>(visitChildren(ctx))};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  if (ctx->any_rvalue()) {
    auto node =
        ast::ArrowRhsNode{std::any_cast<ast::RValueNode>(visitChildren(ctx))};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  if (ctx->make_constructor()) {
    auto node =
        ast::ArrowRhsNode{std::any_cast<ast::MakeNode>(visitChildren(ctx))};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  return {};
}

std::any ASTBuilderVisitor::visitMake_constructor(
    AasmParser::Make_constructorContext *ctx) {
  return ast::MakeNode{
      {.sourcePos = ctx->getSourceInterval().toString()},
      .type{std::any_cast<ast::ObjectTypeNode>(visitChildren(ctx))}};
}

std::any ASTBuilderVisitor::visitCall_instruction(
    AasmParser::Call_instructionContext *ctx) {
  return ast::CallNode{{.sourcePos = ctx->getSourceInterval().toString()},
                       .id{ast::IdNode{.id = ctx->ID()->getText()}}};
}

std::any ASTBuilderVisitor::visitPrint_instruction(
    AasmParser::Print_instructionContext *ctx) {
  return ast::NoRetNode{{.sourcePos = ctx->getSourceInterval().toString()},
                        .op = ast::NoRetOperator::kPrint,
                        .arg = std::any_cast<ast::ArgNode>(visitChildren(ctx))};
}

std::any ASTBuilderVisitor::visitPrintch_instruction(
    AasmParser::Printch_instructionContext *ctx) {
  return ast::NoRetNode{{.sourcePos = ctx->getSourceInterval().toString()},
                        .op = ast::NoRetOperator::kPrintChar,
                        .arg = std::any_cast<ast::ArgNode>(visitChildren(ctx))};
}

std::any ASTBuilderVisitor::visitExit_instruction(
    AasmParser::Exit_instructionContext *ctx) {
  return ast::NoRetNode{{.sourcePos = ctx->getSourceInterval().toString()},
                        .op = ast::NoRetOperator::kExit,
                        .arg = std::any_cast<ast::ArgNode>(visitChildren(ctx))};
}

std::any ASTBuilderVisitor::visitUnary_operator_instruction(
    AasmParser::Unary_operator_instructionContext *ctx) {
  ast::UnaryNode node;
  node.op = std::any_cast<ast::UnaryOperator>(
      visitUnary_operator(ctx->unary_operator()));
  node.lhs = std::any_cast<ast::LValueNode>(visitAny_lvalue(ctx->arg1));
  node.rhs = std::any_cast<ast::ArgNode>(visitAny_argument(ctx->arg2));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any ASTBuilderVisitor::visitBinary_operator_instruction(
    AasmParser::Binary_operator_instructionContext *ctx) {
  ast::BinaryNode node;
  node.op = std::any_cast<ast::BinaryOperator>(
      visitBinary_operator(ctx->binary_operator()));
  node.lhs = std::any_cast<ast::LValueNode>(visitAny_lvalue(ctx->arg1));
  node.arg1 = std::any_cast<ast::ArgNode>(visitAny_argument(ctx->arg2));
  node.arg2 = std::any_cast<ast::ArgNode>(visitAny_argument(ctx->arg3));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any ASTBuilderVisitor::visitMemory_instruction(
    AasmParser::Memory_instructionContext *ctx) {
  ast::MemoryNode node;
  node.op = std::any_cast<ast::MemoryOperator>(
      visitMemory_operator(ctx->memory_operator()));
  node.register_dst =
      std::any_cast<ast::LValueNode>(visitAny_lvalue(ctx->arg1));
  node.memory_location = std::any_cast<
      std::variant<std::monostate, ast::RValueNode, ast::MemberNode>>(
      visitMemory_destination(ctx->arg2));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any
ASTBuilderVisitor::visitUnary_operator(AasmParser::Unary_operatorContext *ctx) {
  if (ctx->getText() == "aneg")
    return ast::UnaryOperator::kANeg;
  if (ctx->getText() == "bneg")
    return ast::UnaryOperator::kBNeg;
  if (ctx->getText() == "lneg")
    return ast::UnaryOperator::kLNeg;
  return {};
}

std::any ASTBuilderVisitor::visitBinary_operator(
    AasmParser::Binary_operatorContext *ctx) {
  if (ctx->getText() == "add")
    return ast::BinaryOperator::kAdd;
  if (ctx->getText() == "sub")
    return ast::BinaryOperator::kSub;
  if (ctx->getText() == "mul")
    return ast::BinaryOperator::kMul;
  if (ctx->getText() == "div")
    return ast::BinaryOperator::kDiv;
  if (ctx->getText() == "mod")
    return ast::BinaryOperator::kMod;
  if (ctx->getText() == "sll")
    return ast::BinaryOperator::kSll;
  if (ctx->getText() == "srl")
    return ast::BinaryOperator::kSrl;
  if (ctx->getText() == "sra")
    return ast::BinaryOperator::kSra;
  if (ctx->getText() == "and")
    return ast::BinaryOperator::kAnd;
  if (ctx->getText() == "or")
    return ast::BinaryOperator::kOr;
  if (ctx->getText() == "xor")
    return ast::BinaryOperator::kXor;
  return {};
}

std::any ASTBuilderVisitor::visitMemory_operator(
    AasmParser::Memory_operatorContext *ctx) {
  if (ctx->getText() == "load")
    return ast::MemoryOperator::kLoad;
  if (ctx->getText() == "store")
    return ast::MemoryOperator::kStore;
  return {};
}

std::any ASTBuilderVisitor::visitMemory_destination(
    AasmParser::Memory_destinationContext *ctx) {
  if (ctx->any_field())
    return std::variant<std::monostate, ast::RValueNode, ast::MemberNode>{
        std::any_cast<ast::MemberNode>(visitChildren(ctx))};
  if (ctx->any_rvalue())
    return std::variant<std::monostate, ast::RValueNode, ast::MemberNode>{
        std::any_cast<ast::RValueNode>(visitChildren(ctx))};
  return {};
}

std::any
ASTBuilderVisitor::visitAny_lvalue(AasmParser::Any_lvalueContext *ctx) {
  auto text = ctx->getText();
  ast::LValueNode node;
  if (text.starts_with("l")) {
    node.category = ast::RegisterCategory::Local;
    node.register_id = std::stoi(text.substr(1));
  } else if (text.starts_with("p")) {
    node.category = ast::RegisterCategory::Param;
    node.register_id = std::stoi(text.substr(1));
  } else if (text.starts_with("op")) {
    node.category = ast::RegisterCategory::OutgoingParam;
    node.register_id = std::stoi(text.substr(2));
  } else if (text.starts_with("rr")) {
    node.category = ast::RegisterCategory::Return;
    node.register_id = std::stoi(text.substr(2));
  } else if (text.starts_with("sr")) {
    node.category = ast::RegisterCategory::Global;
    node.register_id = std::stoi(text.substr(2));
  }
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any
ASTBuilderVisitor::visitAny_argument(AasmParser::Any_argumentContext *ctx) {
  if (ctx->any_rvalue()) {
    auto node = ast::ArgNode(
        std::any_cast<ast::RValueNode>(visitAny_rvalue(ctx->any_rvalue())));
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  if (ctx->any_number()) {
    auto node = ast::ArgNode(
        std::any_cast<ast::ImmediateNode>(visitAny_number(ctx->any_number())));
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  return {};
}

std::any
ASTBuilderVisitor::visitAny_rvalue(AasmParser::Any_rvalueContext *ctx) {
  auto text = ctx->getText();
  ast::RValueNode node;
  if (text.starts_with("l")) {
    node.category = ast::RegisterCategory::Local;
    node.register_id = std::stoi(text.substr(1));
  } else if (text.starts_with("p")) {
    node.category = ast::RegisterCategory::Param;
    node.register_id = std::stoi(text.substr(1));
  } else if (text.starts_with("op")) {
    node.category = ast::RegisterCategory::OutgoingParam;
    node.register_id = std::stoi(text.substr(2));
  } else if (text.starts_with("rr")) {
    node.category = ast::RegisterCategory::Return;
    node.register_id = std::stoi(text.substr(2));
  } else if (text.starts_with("sr")) {
    node.category = ast::RegisterCategory::Global;
    node.register_id = std::stoi(text.substr(2));
  }
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any
ASTBuilderVisitor::visitAny_number(AasmParser::Any_numberContext *ctx) {
  std::string text = ctx->getText();
  if (text.find('.') != std::string::npos) {
    auto node = ast::ImmediateNode(std::stod(ctx->getText()));
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  auto node =
      ast::ImmediateNode(static_cast<int64_t>(std::stol(ctx->getText())));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any ASTBuilderVisitor::visitAny_field(AasmParser::Any_fieldContext *ctx) {
  return ast::MemberNode{
      {.sourcePos = ctx->getSourceInterval().toString()},
      .obj{std::any_cast<ast::RValueNode>(visitAny_rvalue(ctx->any_rvalue()))},
      .type{ast::IdNode{.id = ctx->type_name->getText()}},
      .field{ast::IdNode{.id = ctx->field_name->getText()}}};
}

std::any
ASTBuilderVisitor::visitObject_type(AasmParser::Object_typeContext *ctx) {
  auto text = ctx->getText();
  if (text.starts_with("long")) {
    auto node = ast::ObjectTypeNode{ast::LongNode()};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  } else if (text.starts_with("double")) {
    auto node = ast::ObjectTypeNode{ast::DoubleNode()};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  } else if (text.starts_with("ptr")) {
    auto element_type = std::any_cast<ast::ObjectTypeNode>(
        visitObject_type(ctx->object_type()));
    auto node = ast::ObjectTypeNode{ast::PtrNode{
        .element_type{std::make_shared<ast::ObjectTypeNode>(element_type)}}};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  auto node = ast::ObjectTypeNode{ast::IdNode{.id = ctx->ID()->getText()}};
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any
ASTBuilderVisitor::visitRegister_type(AasmParser::Register_typeContext *ctx) {
  auto text = ctx->getText();
  if (text.starts_with("long")) {
    auto node = ast::RegisterTypeNode{ast::LongNode()};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  } else if (text.starts_with("double")) {
    auto node = ast::RegisterTypeNode{ast::DoubleNode()};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  } else if (text.starts_with("ptr")) {
    auto element_type = std::any_cast<ast::ObjectTypeNode>(
        visitObject_type(ctx->object_type()));
    auto node = ast::RegisterTypeNode{ast::PtrNode{
        .element_type{std::make_shared<ast::ObjectTypeNode>(element_type)}}};
    node.sourcePos = ctx->getSourceInterval().toString();
    return node;
  }
  return {};
}

std::any ASTBuilderVisitor::visitField(AasmParser::FieldContext *ctx) {
  return ast::FieldNode{{.sourcePos = ctx->getSourceInterval().toString()},
                        .id = ast::IdNode{.id = ctx->ID()->getText()},
                        .type{std::any_cast<ast::RegisterTypeNode>(
                            visitRegister_type(ctx->register_type()))}};
}

std::any
ASTBuilderVisitor::visitIf_statement(AasmParser::If_statementContext *ctx) {
  ast::IfNode node;
  node.condition =
      std::any_cast<ast::ArgNode>(visitAny_argument(ctx->any_argument()));
  node.body = std::any_cast<std::vector<std::shared_ptr<ast::InstructionNode>>>(
      visitInstructions(ctx->instructions()));
  for (auto elif_branch : ctx->elif_branch()) {
    node.elifs.push_back(
        std::any_cast<ast::ElifNode>(visitElif_branch(elif_branch)));
  }
  if (ctx->else_branch()) {
    node.else_node =
        std::any_cast<ast::ElseNode>(visitElse_branch(ctx->else_branch()));
  }
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any
ASTBuilderVisitor::visitElif_branch(AasmParser::Elif_branchContext *ctx) {
  ast::ElifNode node;
  node.condition =
      std::any_cast<ast::ArgNode>(visitAny_argument(ctx->any_argument()));
  node.body = std::any_cast<std::vector<std::shared_ptr<ast::InstructionNode>>>(
      visitInstructions(ctx->instructions()));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any
ASTBuilderVisitor::visitElse_branch(AasmParser::Else_branchContext *ctx) {
  ast::ElseNode node;
  node.body = std::any_cast<std::vector<std::shared_ptr<ast::InstructionNode>>>(
      visitInstructions(ctx->instructions()));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}

std::any
ASTBuilderVisitor::visitWhile_loop(AasmParser::While_loopContext *ctx) {
  ast::WhileNode node;
  node.condition =
      std::any_cast<ast::ArgNode>(visitAny_argument(ctx->any_argument()));
  node.body = std::any_cast<std::vector<std::shared_ptr<ast::InstructionNode>>>(
      visitInstructions(ctx->instructions()));
  node.sourcePos = ctx->getSourceInterval().toString();
  return node;
}
} // namespace parser
