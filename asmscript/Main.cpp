#include "Common.h"
#include "Lexer.h"
#include "Parser.h"
#include "Compiler.h"
#include "Runtime.h"

#include <cstdio>
#include <cstring>
#include <iomanip>
#include <iostream>

#include <errno.h>
#include <sys/mman.h>

static int RunFile(const char* filepath);
static void PrintLexResults(std::string_view filePrefix, const std::vector<std::unique_ptr<Token>>& tokens);
static void PrintParseResults(std::string_view filePrefix, const std::unordered_map<std::string, std::vector<std::unique_ptr<Statement>>>& procedures);
static void PrintCompileResults(const std::basic_string<unsigned char>& machineCode, size_t entry);
static void ExecuteCompileResults(const std::basic_string<unsigned char>& machineCode, size_t entry);
static void PrintStatements(std::string_view filePrefix, const std::vector<std::unique_ptr<Statement>>& statements, size_t level = 0);
static void PrintRegister(Register reg);
static void PrintOperation(Operation op);
static void PrintCondition(const Condition& condition);
static void PrintOperand(const Operand& operand);

static bool flag_dumpTokens = false;
static bool flag_dumpAst = false;
static bool flag_dumpCode = false;
static bool flag_noExec = false;

int main(int argc, char* argv[])
{
	if (argc < 2)
	{
		fprintf(stderr,
			"Usage: %s [FLAGS] FILE\n"
			"    --dump-tokens    Dump lexer results\n"
			"    --dump-ast       Dump parser results\n"
			"    --dump-code      Dump machine code\n"
			"    --no-exec        Do not execute compiled code\n",
			argv[0]
		);
		return 1;
	}

	for (int argnum = 1; argnum < argc - 1; ++argnum)
	{
		const char* arg = argv[argnum];
		if (strcmp(arg, "--dump-tokens") == 0) flag_dumpTokens = true;
		else if (strcmp(arg, "--dump-ast") == 0) flag_dumpAst = true;
		else if (strcmp(arg, "--dump-code") == 0) flag_dumpCode = true;
		else if (strcmp(arg, "--no-exec") == 0) flag_noExec = true;
	}

	return RunFile(argv[argc - 1]);
}

static int RunFile(const char* const filepath)
{
	std::string code;
	if (!ReadFile(filepath, code))
	{
		std::cerr << "Couldn't read file " << filepath << '\n';
		return 1;
	}

	std::vector<std::unique_ptr<Token>> tokens;
	Error error = Lex(code.c_str(), tokens);
	if (error)
	{
		fprintf(stderr, "%s:%zu:%zu: Lexer error: %s\n", filepath, error.pos.line, error.pos.col, error.message.c_str());
		return 1;
	}

	if (flag_dumpTokens) PrintLexResults(filepath, tokens);

	std::unordered_map<std::string, std::vector<std::unique_ptr<Statement>>> procedures;
	error = Parse(tokens, procedures);
	if (error)
	{
		fprintf(stderr, "%s:%zu:%zu: Parser error: %s\n", filepath, error.pos.line, error.pos.col, error.message.c_str());
		return 1;
	}

	if (flag_dumpAst) PrintParseResults(filepath, procedures);

	std::basic_string<unsigned char> machineCode;
	size_t entry;
	error = Compile(procedures, machineCode, entry);
	if (error)
	{
		fprintf(stderr, "%s:%zu:%zu: Compiler error: %s\n", filepath, error.pos.line, error.pos.col, error.message.c_str());
		return 1;
	}

	if (flag_dumpCode) PrintCompileResults(machineCode, entry);
	if (!flag_noExec) ExecuteCompileResults(machineCode, entry);

	return 0;
}

static void PrintLexResults(const std::string_view filePrefix, const std::vector<std::unique_ptr<Token>>& tokens)
{
	for (const auto& token : tokens)
	{
		std::cout << filePrefix << ':' << token->pos.line << ':' << token->pos.col << ": ";

		switch (token->tag)
		{
			case TokenTag::RegRax: std::cout << "RegRax"; break;
			case TokenTag::RegRbx: std::cout << "RegRbx"; break;
			case TokenTag::RegRcx: std::cout << "RegRcx"; break;
			case TokenTag::RegRdx: std::cout << "RegRdx"; break;
			case TokenTag::RegRsi: std::cout << "RegRsi"; break;
			case TokenTag::RegRdi: std::cout << "RegRdi"; break;
			case TokenTag::RegRbp: std::cout << "RegRbp"; break;
			case TokenTag::RegR8: std::cout << "RegR8"; break;
			case TokenTag::RegR9: std::cout << "RegR9"; break;
			case TokenTag::RegR10: std::cout << "RegR10"; break;
			case TokenTag::RegR11: std::cout << "RegR11"; break;
			case TokenTag::RegR12: std::cout << "RegR12"; break;
			case TokenTag::RegR13: std::cout << "RegR13"; break;
			case TokenTag::RegR14: std::cout << "RegR14"; break;
			case TokenTag::RegR15: std::cout << "RegR15"; break;
			case TokenTag::RegXmm0: std::cout << "RegXmm0"; break;
			case TokenTag::RegXmm1: std::cout << "RegXmm1"; break;
			case TokenTag::RegXmm2: std::cout << "RegXmm2"; break;
			case TokenTag::RegXmm3: std::cout << "RegXmm3"; break;
			case TokenTag::RegXmm4: std::cout << "RegXmm4"; break;
			case TokenTag::RegXmm5: std::cout << "RegXmm5"; break;
			case TokenTag::RegXmm6: std::cout << "RegXmm6"; break;
			case TokenTag::RegXmm7: std::cout << "RegXmm7"; break;
			case TokenTag::RegXmm8: std::cout << "RegXmm8"; break;
			case TokenTag::RegXmm9: std::cout << "RegXmm9"; break;
			case TokenTag::RegXmm10: std::cout << "RegXmm10"; break;
			case TokenTag::RegXmm11: std::cout << "RegXmm11"; break;
			case TokenTag::RegXmm12: std::cout << "RegXmm12"; break;
			case TokenTag::RegXmm13: std::cout << "RegXmm13"; break;
			case TokenTag::RegXmm14: std::cout << "RegXmm14"; break;
			case TokenTag::RegXmm15: std::cout << "RegXmm15"; break;
			case TokenTag::KeyBranch: std::cout << "KeyBranch"; break;
			case TokenTag::KeyBreak: std::cout << "KeyBreak"; break;
			case TokenTag::KeyContinue: std::cout << "KeyContinue"; break;
			case TokenTag::KeyElse: std::cout << "KeyElse"; break;
			case TokenTag::KeyIf: std::cout << "KeyIf"; break;
			case TokenTag::KeyLoop: std::cout << "KeyLoop"; break;
			case TokenTag::KeyMacro: std::cout << "KeyMacro"; break;
			case TokenTag::KeyPop: std::cout << "KeyPop"; break;
			case TokenTag::KeyProc: std::cout << "KeyProc"; break;
			case TokenTag::KeyPush: std::cout << "KeyPush"; break;
			case TokenTag::KeyReturn: std::cout << "KeyReturn"; break;
			case TokenTag::KeyVal: std::cout << "KeyVal"; break;
			case TokenTag::KeyVar: std::cout << "KeyVar"; break;
			case TokenTag::BracketOpen: std::cout << "BracketOpen"; break;
			case TokenTag::BracketClose: std::cout << "BracketClose"; break;
			case TokenTag::ParenOpen: std::cout << "ParenOpen"; break;
			case TokenTag::ParenClose: std::cout << "ParenClose"; break;
			case TokenTag::BraceOpen: std::cout << "BraceOpen"; break;
			case TokenTag::BraceClose: std::cout << "BraceClose"; break;
			case TokenTag::Plus: std::cout << "Plus"; break;
			case TokenTag::Minus: std::cout << "Minus"; break;
			case TokenTag::Star: std::cout << "Star"; break;
			case TokenTag::Slash: std::cout << "Slash"; break;
			case TokenTag::Percent: std::cout << "Percent"; break;
			case TokenTag::Ampersand: std::cout << "Ampersand"; break;
			case TokenTag::Pipe: std::cout << "Pipe"; break;
			case TokenTag::Caret: std::cout << "Caret"; break;
			case TokenTag::PlusEquals: std::cout << "PlusEquals"; break;
			case TokenTag::MinusEquals: std::cout << "MinusEquals"; break;
			case TokenTag::StarEquals: std::cout << "StarEquals"; break;
			case TokenTag::SlashEquals: std::cout << "SlashEquals"; break;
			case TokenTag::PercentEquals: std::cout << "PercentEquals"; break;
			case TokenTag::AmpersandEquals: std::cout << "AmpersandEquals"; break;
			case TokenTag::PipeEquals: std::cout << "PipeEquals"; break;
			case TokenTag::CaretEquals: std::cout << "CaretEquals"; break;
			case TokenTag::Equals: std::cout << "Equals"; break;
			case TokenTag::LessThan: std::cout << "LessThan"; break;
			case TokenTag::GreaterThan: std::cout << "GreaterThan"; break;
			case TokenTag::LessEquals: std::cout << "LessEquals"; break;
			case TokenTag::GreaterEquals: std::cout << "GreaterEquals"; break;
			case TokenTag::EqualsEquals: std::cout << "EqualsEquals"; break;
			case TokenTag::NotEquals: std::cout << "NotEquals"; break;
			case TokenTag::Hash: std::cout << "Hash"; break;
			case TokenTag::Shl: std::cout << "Shl"; break;
			case TokenTag::Shr: std::cout << "Shr"; break;
			case TokenTag::Comma: std::cout << "Comma"; break;
			case TokenTag::Semicolon: std::cout << "Semicolon"; break;
			case TokenTag::Number: std::cout << "Number " << static_cast<NumberToken*>(token.get())->value; break;
			case TokenTag::Identifier: std::cout << "Identifier " << static_cast<IdentifierToken*>(token.get())->name; break;
			case TokenTag::String: std::cout << "String " << static_cast<StringToken*>(token.get())->value; break;
			case TokenTag::Eof: std::cout << "EOF"; break;
		}

		std::cout << '\n';
	}
}

static void PrintParseResults(const std::string_view filePrefix, const std::unordered_map<std::string, std::vector<std::unique_ptr<Statement>>>& procedures)
{
	for (const auto& [name, statements] : procedures)
	{
		std::cout << "PROCEDURE " << name << "\n\n";

		PrintStatements(filePrefix, statements);

		std::cout << '\n';
	}
}

static void PrintCompileResults(const std::basic_string<unsigned char>& machineCode, const size_t entry)
{
	printf("Entry point is at 0x%016zX\n", entry);
	printf(
		"Runtime library function would be at:\n"
		"\t0x%016zX: void RtPrint(int64_t)\n"
		"\t0x%016zX: void RtPrint(const char*, size_t)\n",
		(size_t)(void (*)(int64_t))(&RtPrint),
		(size_t)(void (*)(const char*, size_t))(&RtPrint)
	);

	for (const unsigned char c : machineCode)
	{
		printf("%02X ", c);
	}

	puts("");
}

static void ExecuteCompileResults(const std::basic_string<unsigned char>& machineCode, size_t entry)
{
	const size_t len = machineCode.length();
	void* mem = mmap(NULL, len, PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
	if (mem == MAP_FAILED)
	{
		fprintf(stderr, "Mapping memory failed with errno %d\n", errno);
		return;
	}
	memcpy(mem, machineCode.data(), len);
	mprotect(mem, len, PROT_EXEC | PROT_READ);

	char* entryPtr = static_cast<char*>(mem) + entry;
	void (*main)();
	memcpy(&main, &entryPtr, 8);
	main();

	munmap(mem, len);
}

static void PrintStatements(const std::string_view filePrefix, const std::vector<std::unique_ptr<Statement>>& statements, const size_t level)
{
	for (const auto& statement : statements)
	{
		for (size_t i = 0; i < level; ++i) std::cout << '\t';

		switch (statement->tag)
		{
		case StatementTag::Assignment:
		{
			auto stmt = static_cast<AssignmentStatement*>(statement.get());
			std::cout << "Assignment ";
			PrintRegister(stmt->dest);
			std::cout << " = ";
			PrintOperand(*stmt->source);
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		case StatementTag::Shorthand:
		{
			auto stmt = static_cast<ShorthandStatement*>(statement.get());
			std::cout << "Shorthand ";
			PrintRegister(stmt->dest);
			std::cout << ' ';
			PrintOperation(stmt->op);
			std::cout << "= ";
			PrintOperand(*stmt->source);
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		case StatementTag::Longhand:
		{
			auto stmt = static_cast<LonghandStatement*>(statement.get());
			std::cout << "Longhand ";
			PrintRegister(stmt->dest);
			std::cout << " = ";
			PrintOperand(*stmt->sourceA);
			std::cout << ' ';
			PrintOperation(stmt->op);
			std::cout << ' ';
			PrintOperand(*stmt->sourceB);
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		case StatementTag::Loop:
		{
			auto stmt = static_cast<LoopStatement*>(statement.get());
			std::cout << "Loop";
			if (stmt->condition.has_value())
			{
				std::cout << " (";
				PrintCondition(*stmt->condition);
				std::cout << ")";
			}
			std::cout << '\n';
			PrintStatements(filePrefix, stmt->statements, level + 1);
			continue;
		}
		case StatementTag::Branch:
		{
			auto stmt = static_cast<BranchStatement*>(statement.get());
			std::cout << "Branch (";
			PrintCondition(*stmt->condition);
			std::cout << ")\n";
			PrintStatements(filePrefix, stmt->statements, level + 1);
			std::cout << "Else\n";
			PrintStatements(filePrefix, stmt->elseBlock, level + 1);
			continue;
		}
		case StatementTag::Break:
		{
			std::cout << "Break";
			if (statement->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*statement->condition);
			}
			break;
		}
		case StatementTag::Continue:
		{
			std::cout << "Continue";
			if (statement->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*statement->condition);
			}
			break;
		}
		case StatementTag::Return:
		{
			std::cout << "Return";
			if (statement->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*statement->condition);
			}
			break;
		}
		case StatementTag::Call:
		{
			auto stmt = static_cast<CallStatement*>(statement.get());
			std::cout << "Call " << stmt->name;
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		case StatementTag::Stdout:
		{
			auto stmt = static_cast<StdoutStatement*>(statement.get());
			std::cout << "Stdout ";
			PrintOperand(*stmt->source);
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		case StatementTag::StdoutText:
		{
			auto stmt = static_cast<StdoutTextStatement*>(statement.get());
			std::cout << "StdoutText ";
			std::cout << stmt->text;
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		case StatementTag::Push:
		{
			auto stmt = static_cast<RegisterStatement*>(statement.get());
			std::cout << "Push ";
			PrintRegister(stmt->reg);
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		case StatementTag::Pop:
		{
			auto stmt = static_cast<RegisterStatement*>(statement.get());
			std::cout << "Pop ";
			PrintRegister(stmt->reg);
			if (stmt->condition.has_value())
			{
				std::cout << " if ";
				PrintCondition(*stmt->condition);
			}
			break;
		}
		}
		std::cout << '\n';
	}
}

static void PrintRegister(const Register reg)
{
	switch (reg)
	{
		case Register::rax: std::cout << "rax"; break;
		case Register::rbx: std::cout << "rbx"; break;
		case Register::rcx: std::cout << "rcx"; break;
		case Register::rdx: std::cout << "rdx"; break;
		case Register::rsi: std::cout << "rsi"; break;
		case Register::rdi: std::cout << "rdi"; break;
		case Register::rbp: std::cout << "rbp"; break;
		case Register::r8: std::cout << "r8"; break;
		case Register::r9: std::cout << "r9"; break;
		case Register::r10: std::cout << "r10"; break;
		case Register::r11: std::cout << "r11"; break;
		case Register::r12: std::cout << "r12"; break;
		case Register::r13: std::cout << "r13"; break;
		case Register::r14: std::cout << "r14"; break;
		case Register::r15: std::cout << "r15"; break;
	}
}

static void PrintOperation(const Operation op)
{
	switch (op)
	{
		case Operation::Add: std::cout << '+'; break;
		case Operation::Sub: std::cout << '-'; break;
		case Operation::Mul: std::cout << '*'; break;
		case Operation::Div: std::cout << '/'; break;
		case Operation::Mod: std::cout << '%'; break;
		case Operation::And: std::cout << '&'; break;
		case Operation::Or: std::cout << '|'; break;
		case Operation::Xor: std::cout << '^'; break;
	}
}

static void PrintCondition(const Condition& condition)
{
	PrintOperand(*condition.a);
	switch (condition.comp)
	{
		case Comparison::LessThan: std::cout << " < "; break;
		case Comparison::LessEquals: std::cout << " <= "; break;
		case Comparison::GreaterThan: std::cout << " > "; break;
		case Comparison::GreaterEquals: std::cout << " >= "; break;
		case Comparison::Equals: std::cout << " == "; break;
		case Comparison::NotEquals: std::cout << " != "; break;
	}
	PrintOperand(*condition.b);
}

static void PrintOperand(const Operand& operand)
{
	switch (operand.tag)
	{
		case OperandTag::Register: PrintRegister(static_cast<const RegisterOperand&>(operand).reg); break;
		case OperandTag::Immediate: std::cout << static_cast<const ImmediateOperand&>(operand).value; break;
	}
}
