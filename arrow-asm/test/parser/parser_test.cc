#include "gtest/gtest.h"

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

#include "src/parser/parser.hh"

namespace {
TEST(ParserTest, ParserTestSimple) {
    std::ifstream stream("./test/test_programs/simple.aasm", std::ios::in);
    std::stringstream ss;
    ss << stream.rdbuf();

    auto ast = parser::ParseFullProgram(ss.str());
    EXPECT_EQ(ast.statements.size(), 1);
    EXPECT_TRUE(std::holds_alternative<ast::FunctionNode>(*ast.statements[0]));

    auto func = std::get<ast::FunctionNode>(*ast.statements[0]);
    EXPECT_EQ(func.id.id, "main");
    EXPECT_TRUE(std::holds_alternative<ast::NoRetNode>(*func.body[0]));
}

TEST(ParserTest, ParserTestMemory) {
    std::ifstream stream("./test/test_programs/memory.aasm", std::ios::in);
    std::stringstream ss;
    ss << stream.rdbuf();

    auto ast = parser::ParseFullProgram(ss.str());
    EXPECT_EQ(ast.statements.size(), 1);
    EXPECT_TRUE(std::holds_alternative<ast::FunctionNode>(*ast.statements[0]));

    auto func = std::get<ast::FunctionNode>(*ast.statements[0]);
    EXPECT_EQ(func.id.id, "main");
    EXPECT_EQ(func.body.size(), 2);
    EXPECT_TRUE(std::holds_alternative<ast::MemoryNode>(*func.body[0]));
    EXPECT_TRUE(std::holds_alternative<ast::MemoryNode>(*func.body[1]));
    
    auto load_inst = std::get<ast::MemoryNode>(*func.body[0]);
    EXPECT_EQ(load_inst.op, ast::MemoryOperator::kLoad);
    EXPECT_TRUE(std::holds_alternative<ast::MemberNode>(load_inst.memory_location));
    EXPECT_EQ(std::get<ast::MemberNode>(load_inst.memory_location).obj.category, ast::RegisterCategory::Local);
    EXPECT_EQ(std::get<ast::MemberNode>(load_inst.memory_location).obj.register_id, 1);
    EXPECT_EQ(std::get<ast::MemberNode>(load_inst.memory_location).field.id, "a");
    
    auto store_inst = std::get<ast::MemoryNode>(*func.body[1]);
    EXPECT_EQ(store_inst.op, ast::MemoryOperator::kStore);
    EXPECT_TRUE(std::holds_alternative<ast::RValueNode>(store_inst.memory_location));
    EXPECT_EQ(std::get<ast::RValueNode>(store_inst.memory_location).category, ast::RegisterCategory::Local);
    EXPECT_EQ(std::get<ast::RValueNode>(store_inst.memory_location).register_id, 2);
}

TEST(ParserTest, ParserTestArrow) {
    std::ifstream stream("./test/test_programs/arrow.aasm", std::ios::in);
    std::stringstream ss;
    ss << stream.rdbuf();

    auto ast = parser::ParseFullProgram(ss.str());
    EXPECT_EQ(ast.statements.size(), 1);
    EXPECT_TRUE(std::holds_alternative<ast::FunctionNode>(*ast.statements[0]));

    auto func = std::get<ast::FunctionNode>(*ast.statements[0]);
    EXPECT_EQ(func.id.id, "main");
    EXPECT_EQ(func.body.size(), 2);
    EXPECT_TRUE(std::holds_alternative<ast::ArrowInstNode>(*func.body[0]));
    EXPECT_TRUE(std::holds_alternative<ast::ArrowInstNode>(*func.body[1]));
}

TEST(ParserTest, ParserTestTypes) {
    std::ifstream stream("./test/test_programs/types.aasm", std::ios::in);
    std::stringstream ss;
    ss << stream.rdbuf();

    auto ast = parser::ParseFullProgram(ss.str());
    EXPECT_EQ(ast.statements.size(), 2);
    EXPECT_TRUE(std::holds_alternative<ast::TypeNode>(*ast.statements[0]));
    EXPECT_TRUE(std::holds_alternative<ast::TypeNode>(*ast.statements[0]));
}

TEST(ParserTest, ParserTestBranch) {
    std::ifstream stream("./test/test_programs/branch.aasm", std::ios::in);
    std::stringstream ss;
    ss << stream.rdbuf();

    auto ast = parser::ParseFullProgram(ss.str());
}

TEST(ParserTest, ParserTestComplex) {
    std::ifstream stream("./test/test_programs/test_complex_type.aasm", std::ios::in);
    std::stringstream ss;
    ss << stream.rdbuf();

    auto ast = parser::ParseFullProgram(ss.str());
}


}