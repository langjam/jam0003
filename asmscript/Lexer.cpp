#include "Common.h"
#include "Lexer.h"

#include <cstdint>
#include <cstdlib>
#include <iostream>

using namespace std::literals;

struct CodePtr {
	const char* ptr;
	CodePos pos;

	CodePtr(const char* const ptr) : ptr{ptr}, pos{1, 1} {}

	const char& operator[](const size_t index) const { return ptr[index]; }

	CodePtr& operator+=(const size_t count)
	{
		for (size_t i = 0; i < count; ++i, ++ptr)
		{
			const char c = *ptr;
			if (c == '\n')
			{
				pos.line += 1;
				pos.col = 1;
			}
			else
			{
				pos.col += 1;
			}
		}
		return *this;
	}
};

constexpr size_t KEYWORD_COUNT = 44;
constexpr std::string_view KEYWORDS[KEYWORD_COUNT] = {
	"rax"sv,
	"rbx"sv,
	"rcx"sv,
	"rdx"sv,
	"rsi"sv,
	"rdi"sv,
	"rbp"sv,
	"r8"sv,
	"r9"sv,
	"r10"sv,
	"r11"sv,
	"r12"sv,
	"r13"sv,
	"r14"sv,
	"r15"sv,
	"xmm0"sv,
	"xmm1"sv,
	"xmm2"sv,
	"xmm3"sv,
	"xmm4"sv,
	"xmm5"sv,
	"xmm6"sv,
	"xmm7"sv,
	"xmm8"sv,
	"xmm9"sv,
	"xmm10"sv,
	"xmm11"sv,
	"xmm12"sv,
	"xmm13"sv,
	"xmm14"sv,
	"xmm15"sv,
	"branch"sv,
	"break"sv,
	"continue"sv,
	"else"sv,
	"if"sv,
	"loop"sv,
	"macro"sv,
	"pop"sv,
	"proc"sv,
	"push"sv,
	"return"sv,
	"val"sv,
	"var"sv,
};

static bool lexerSuccess;

static void SkipWhitespace(CodePtr& ptr);
static bool IsIdentifierStart(char c);
static bool IsIdentifierMiddle(char c);
static bool IsDigit(char c);
[[nodiscard]] static Error LexComment(CodePtr& ptr);
[[nodiscard]] static Error LexIdentifierOrKeyword(CodePtr& ptr, std::unique_ptr<Token>& out);
[[nodiscard]] static Error LexNumber(CodePtr& ptr, std::unique_ptr<Token>& out);
[[nodiscard]] static Error LexString(CodePtr& ptr, std::unique_ptr<Token>& out);

[[nodiscard]] Error Lex(const char* const code, std::vector<std::unique_ptr<Token>>& tokens)
{
	std::unique_ptr<Token> token;
	CodePtr codePtr{code};

	while (true)
	{
		// whitespace

		SkipWhitespace(codePtr);

		if (codePtr[0] == '\0')
		{
			tokens.emplace_back(std::make_unique<Token>(TokenTag::Eof, codePtr.pos));
			return Error::None;
		}

		// comment

		TRY(LexComment(codePtr));
		if (lexerSuccess)
		{
			continue;
		}

		// identifier or keyword

		TRY(LexIdentifierOrKeyword(codePtr, token));
		if (lexerSuccess)
		{
			tokens.emplace_back(std::move(token));
			continue;
		}

		// number

		TRY(LexNumber(codePtr, token));
		if (lexerSuccess)
		{
			tokens.emplace_back(std::move(token));
			continue;
		}

		// string

		TRY(LexString(codePtr, token));
		if (lexerSuccess)
		{
			tokens.emplace_back(std::move(token));
			continue;
		}

		// simple tokens (2 chars)

		if (codePtr[0] != '\0')
		{
			const uint16_t val = (codePtr[0] << 8) | codePtr[1];
			switch (val)
			{
				case 0x2B3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::PlusEquals, codePtr.pos));      codePtr += 2; continue;
				case 0x2D3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::MinusEquals, codePtr.pos));     codePtr += 2; continue;
				case 0x2A3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::StarEquals, codePtr.pos));      codePtr += 2; continue;
				case 0x2F3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::SlashEquals, codePtr.pos));     codePtr += 2; continue;
				case 0x253D: tokens.emplace_back(std::make_unique<Token>(TokenTag::PercentEquals, codePtr.pos));   codePtr += 2; continue;
				case 0x263D: tokens.emplace_back(std::make_unique<Token>(TokenTag::AmpersandEquals, codePtr.pos)); codePtr += 2; continue;
				case 0x7C3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::PipeEquals, codePtr.pos));      codePtr += 2; continue;
				case 0x5E3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::CaretEquals, codePtr.pos));     codePtr += 2; continue;
				case 0x3C3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::LessEquals, codePtr.pos));      codePtr += 2; continue;
				case 0x3E3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::GreaterEquals, codePtr.pos));   codePtr += 2; continue;
				case 0x3D3D: tokens.emplace_back(std::make_unique<Token>(TokenTag::EqualsEquals, codePtr.pos));    codePtr += 2; continue;
				case 0x213D: tokens.emplace_back(std::make_unique<Token>(TokenTag::NotEquals, codePtr.pos));       codePtr += 2; continue;
				case 0x3C3C: tokens.emplace_back(std::make_unique<Token>(TokenTag::Shl, codePtr.pos));             codePtr += 2; continue;
				case 0x3E3E: tokens.emplace_back(std::make_unique<Token>(TokenTag::Shr, codePtr.pos));             codePtr += 2; continue;
			}
		}

		// simple tokens (1 char)

		switch (codePtr[0])
		{
			case '[': tokens.emplace_back(std::make_unique<Token>(TokenTag::BracketOpen, codePtr.pos));  codePtr += 1; continue;
			case ']': tokens.emplace_back(std::make_unique<Token>(TokenTag::BracketClose, codePtr.pos)); codePtr += 1; continue;
			case '(': tokens.emplace_back(std::make_unique<Token>(TokenTag::ParenOpen, codePtr.pos));    codePtr += 1; continue;
			case ')': tokens.emplace_back(std::make_unique<Token>(TokenTag::ParenClose, codePtr.pos));   codePtr += 1; continue;
			case '{': tokens.emplace_back(std::make_unique<Token>(TokenTag::BraceOpen, codePtr.pos));    codePtr += 1; continue;
			case '}': tokens.emplace_back(std::make_unique<Token>(TokenTag::BraceClose, codePtr.pos));   codePtr += 1; continue;
			case '+': tokens.emplace_back(std::make_unique<Token>(TokenTag::Plus, codePtr.pos));         codePtr += 1; continue;
			case '-': tokens.emplace_back(std::make_unique<Token>(TokenTag::Minus, codePtr.pos));        codePtr += 1; continue;
			case '*': tokens.emplace_back(std::make_unique<Token>(TokenTag::Star, codePtr.pos));         codePtr += 1; continue;
			case '/': tokens.emplace_back(std::make_unique<Token>(TokenTag::Slash, codePtr.pos));        codePtr += 1; continue;
			case '%': tokens.emplace_back(std::make_unique<Token>(TokenTag::Percent, codePtr.pos));      codePtr += 1; continue;
			case '&': tokens.emplace_back(std::make_unique<Token>(TokenTag::Ampersand, codePtr.pos));    codePtr += 1; continue;
			case '|': tokens.emplace_back(std::make_unique<Token>(TokenTag::Pipe, codePtr.pos));         codePtr += 1; continue;
			case '^': tokens.emplace_back(std::make_unique<Token>(TokenTag::Caret, codePtr.pos));        codePtr += 1; continue;
			case '=': tokens.emplace_back(std::make_unique<Token>(TokenTag::Equals, codePtr.pos));       codePtr += 1; continue;
			case '<': tokens.emplace_back(std::make_unique<Token>(TokenTag::LessThan, codePtr.pos));     codePtr += 1; continue;
			case '>': tokens.emplace_back(std::make_unique<Token>(TokenTag::GreaterThan, codePtr.pos));  codePtr += 1; continue;
			case '#': tokens.emplace_back(std::make_unique<Token>(TokenTag::Hash, codePtr.pos));         codePtr += 1; continue;
			case ',': tokens.emplace_back(std::make_unique<Token>(TokenTag::Comma, codePtr.pos));        codePtr += 1; continue;
			case ';': tokens.emplace_back(std::make_unique<Token>(TokenTag::Semicolon, codePtr.pos));    codePtr += 1; continue;
			default: return Error{"Unrecognized token.", codePtr.pos};
		}
	}
}

static void SkipWhitespace(CodePtr& ptr)
{
	while (true)
	{
		const char c = ptr[0];
		switch (c)
		{
		case '\t':
		case '\r':
		case '\n':
		case ' ':
			ptr += 1;
			break;
		default:
			return;
		}
	}
}

static bool IsIdentifierStart(const char c)
{
	return (c >= 'A' && c <= 'Z')
		|| c == '_'
		|| (c >= 'a' && c <= 'z');
}

static bool IsIdentifierMiddle(const char c)
{
	return (c >= '0' && c <= '9')
		|| (c >= 'A' && c <= 'Z')
		|| c == '_'
		|| (c >= 'a' && c <= 'z');
}

static bool IsDigit(const char c)
{
	return c >= '0' && c <= '9';
}

[[nodiscard]] static Error LexComment(CodePtr& ptr)
{
	if (ptr[0] != '/' || ptr[1] != '/')
	{
		lexerSuccess = false;
		return Error::None;
	}

	ptr += 2;

	while (true)
	{
		switch (ptr[0])
		{
		case '\0':
		case '\n':
			ptr += 1;
			lexerSuccess = true;
			return Error::None;
		default:
			ptr += 1;
			break;
		}
	}
}

[[nodiscard]] static Error LexIdentifierOrKeyword(CodePtr& ptr, std::unique_ptr<Token>& out)
{
	if (!IsIdentifierStart(ptr[0]))
	{
		lexerSuccess = false;
		return Error::None;
	}

	const CodePos pos = ptr.pos;

	const char* const start = &ptr[0];

	size_t length = 0;
	do
	{
		length += 1;
		ptr += 1;
	} while (IsIdentifierMiddle(ptr[0]));

	const std::string_view text{start, length};

	// keyword

	for (size_t i = 0; i < KEYWORD_COUNT; ++i)
	{
		if (text != KEYWORDS[i]) continue;

		lexerSuccess = true;
		out = std::make_unique<Token>(static_cast<TokenTag>(i), pos);
		return Error::None;
	}

	// identifier

	lexerSuccess = true;
	out = std::make_unique<IdentifierToken>(std::string{text}, pos);
	return Error::None;
}

[[nodiscard]] static Error LexNumber(CodePtr& ptr, std::unique_ptr<Token>& out)
{
	if (!IsDigit(ptr[0]))
	{
		lexerSuccess = false;
		return Error::None;
	}

	const CodePos pos = ptr.pos;

	const char* const start = &ptr[0];

	ptr += 1;

	while (true)
	{
		if (!IsDigit(ptr[0])) break;
		ptr += 1;
	}

	lexerSuccess = true;
	out = std::make_unique<NumberToken>(atoll(start), pos);
	return Error::None;
}

[[nodiscard]] static Error LexString(CodePtr& ptr, std::unique_ptr<Token>& out)
{
	if (ptr[0] != '"')
	{
		lexerSuccess = false;
		return Error::None;
	}

	const CodePos pos = ptr.pos;

	ptr += 1;

	std::string value;

	while (true)
	{
		switch (ptr[0])
		{
		case '\0':
			return Error{Format("Unexpected EOF, missing \" to close \" (at line %zu, column %zu).", pos.line, pos.col), ptr.pos};
		case '"':
			ptr += 1;

			out = std::make_unique<StringToken>(std::move(value), pos);
			lexerSuccess = true;
			return Error::None;
		case '\\':
			ptr += 1;

			switch (ptr[0])
			{
			case '\0':
				return Error{Format("Unexpected EOF at escape sequence."), ptr.pos};
			case '\\':
				value.push_back('\\');
				break;
			case '0':
				value.push_back('\0');
				break;
			case 'r':
				value.push_back('\r');
				break;
			case 'n':
				value.push_back('\n');
				break;
			case 't':
				value.push_back('\t');
				break;
			default:
				return Error{Format("Invalid escape sequence \\%c.", ptr[0]), ptr.pos};
			}

			ptr += 1;
			break;
		default:
			value.push_back(ptr[0]);
			ptr += 1;
			break;
		}
	}
}
