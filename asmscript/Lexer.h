#pragma once

#include "CodePos.h"
#include "Error.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <utility>
#include <vector>

enum class TokenTag {
	RegRax,
	RegRbx,
	RegRcx,
	RegRdx,
	RegRsi,
	RegRdi,
	RegRbp,
	RegR8,
	RegR9,
	RegR10,
	RegR11,
	RegR12,
	RegR13,
	RegR14,
	RegR15,

	RegXmm0,
	RegXmm1,
	RegXmm2,
	RegXmm3,
	RegXmm4,
	RegXmm5,
	RegXmm6,
	RegXmm7,
	RegXmm8,
	RegXmm9,
	RegXmm10,
	RegXmm11,
	RegXmm12,
	RegXmm13,
	RegXmm14,
	RegXmm15,

	KeyBranch,
	KeyBreak,
	KeyContinue,
	KeyElse,
	KeyIf,
	KeyLoop,
	KeyMacro,
	KeyPop,
	KeyProc,
	KeyPush,
	KeyReturn,
	KeyVal,
	KeyVar,

	BracketOpen,     // [
	BracketClose,    // ]
	ParenOpen,       // (
	ParenClose,      // )
	BraceOpen,       // {
	BraceClose,      // }

	Plus,            // +
	Minus,           // -
	Star,            // *
	Slash,           // /
	Percent,         // %
	Ampersand,       // &
	Pipe,            // |
	Caret,           // ^

	PlusEquals,      // +=
	MinusEquals,     // -=
	StarEquals,      // *=
	SlashEquals,     // /=
	PercentEquals,   // %=
	AmpersandEquals, // &=
	PipeEquals,      // |=
	CaretEquals,     // ^=

	Equals,          // =

	LessThan,        // <
	GreaterThan,     // >
	LessEquals,      // <=
	GreaterEquals,   // >=
	EqualsEquals,    // ==
	NotEquals,       // !=

	Hash,            // #
	Shl,             // <<
	Shr,             // >>
	Comma,           // ,
	Semicolon,       // ;

	Number,
	Identifier,
	String,

	Eof,
};

struct Token {
	TokenTag tag;
	CodePos pos;

	Token(const TokenTag tag, const CodePos pos) : tag{tag}, pos{pos} {}
	virtual ~Token() = default;
};

struct NumberToken : public Token {
	int64_t value;

	NumberToken(const int64_t value, const CodePos pos) : Token{TokenTag::Number, pos}, value{value} {}
};

struct IdentifierToken : public Token {
	std::string name;

	IdentifierToken(std::string name, const CodePos pos) : Token{TokenTag::Identifier, pos}, name{std::move(name)} {}
};

struct StringToken : public Token {
	std::string value;

	StringToken(std::string value, const CodePos pos) : Token{TokenTag::String, pos}, value{std::move(value)} {}
};

[[nodiscard]] Error Lex(const char* code, std::vector<std::unique_ptr<Token>>& tokens);
