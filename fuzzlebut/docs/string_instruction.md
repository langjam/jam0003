# Directinal Regex
`"((\n)?(\r)?(\t)?(\0)?(\")?.)*"`
## TLDR
Any character between two quotes, including escapes.

# Description
Creates a string, pushes all characters in the source until a matching unescaped double quote is reached.

## Valid Escapes
- `\n`
- `\r`
- `\t`
- `\0`
- `\"`
