#include "src/parser/AasmBaseVisitor.h"

#include "src/ast/ast.hh"

namespace parser {
class ASTBuilderVisitor : public AasmBaseVisitor {
public:
  virtual std::any visitProgram(AasmParser::ProgramContext *ctx);
  virtual std::any
  visitFunction_definition(AasmParser::Function_definitionContext *ctx);
  virtual std::any visitParameter_list(AasmParser::Parameter_listContext *ctx);
  virtual std::any
  visitType_definition(AasmParser::Type_definitionContext *ctx);
  virtual std::any visitInstruction(AasmParser::InstructionContext *ctx);
  virtual std::any visitInstructions(AasmParser::InstructionsContext *ctx);
  virtual std::any
  visitNo_arg_instruction(AasmParser::No_arg_instructionContext *ctx);
  virtual std::any
  visitArrow_instruction(AasmParser::Arrow_instructionContext *ctx);
  virtual std::any visitArrow_lhs(AasmParser::Arrow_lhsContext *ctx);
  virtual std::any visitArrow_rhs(AasmParser::Arrow_rhsContext *ctx);
  virtual std::any
  visitMake_constructor(AasmParser::Make_constructorContext *ctx);
  virtual std::any
  visitCall_instruction(AasmParser::Call_instructionContext *ctx);
  virtual std::any
  visitPrint_instruction(AasmParser::Print_instructionContext *ctx);
  virtual std::any
  visitPrintch_instruction(AasmParser::Printch_instructionContext *ctx);
  virtual std::any
  visitExit_instruction(AasmParser::Exit_instructionContext *ctx);
  virtual std::any visitUnary_operator_instruction(
      AasmParser::Unary_operator_instructionContext *ctx);
  virtual std::any visitBinary_operator_instruction(
      AasmParser::Binary_operator_instructionContext *ctx);
  virtual std::any
  visitMemory_instruction(AasmParser::Memory_instructionContext *ctx);
  virtual std::any visitUnary_operator(AasmParser::Unary_operatorContext *ctx);
  virtual std::any
  visitBinary_operator(AasmParser::Binary_operatorContext *ctx);
  virtual std::any
  visitMemory_operator(AasmParser::Memory_operatorContext *ctx);
  virtual std::any
  visitMemory_destination(AasmParser::Memory_destinationContext *ctx);
  virtual std::any visitAny_argument(AasmParser::Any_argumentContext *ctx);
  virtual std::any visitAny_lvalue(AasmParser::Any_lvalueContext *ctx);
  virtual std::any visitAny_rvalue(AasmParser::Any_rvalueContext *ctx);
  virtual std::any visitAny_number(AasmParser::Any_numberContext *ctx);
  virtual std::any visitAny_field(AasmParser::Any_fieldContext *ctx);
  virtual std::any visitObject_type(AasmParser::Object_typeContext *ctx);
  virtual std::any visitRegister_type(AasmParser::Register_typeContext *ctx);
  virtual std::any visitField(AasmParser::FieldContext *ctx);
  virtual std::any visitIf_statement(AasmParser::If_statementContext *ctx);
  virtual std::any visitElif_branch(AasmParser::Elif_branchContext *ctx);
  virtual std::any visitElse_branch(AasmParser::Else_branchContext *ctx);
  virtual std::any visitWhile_loop(AasmParser::While_loopContext *ctx);
};
} // namespace parser
