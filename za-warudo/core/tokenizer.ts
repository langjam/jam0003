export type Token = SymbolToken | PercentToken | SizeToken | ColorToken;

interface SymbolToken {
  type: "symbol";
  value: string;
}

interface PercentToken {
  type: "percent";
  value: number;
}

interface SizeToken {
  type: "size";
  first: number;
  second: number;
}

interface ColorToken {
  type: "color";
  value: string;
}

export function tokenize(input: string): Token[][] {
  const lines = input.split("\n");
  const tokens: Token[][] = [];
  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed) {
      const words = trimmed.split(/\s+/);
      const lineTokens: Token[] = [];
      for (const word of words) {
        const [token, stop] = tokenizeWord(word);
        if (stop) {
          break;
        }
        lineTokens.push(token);
      }
      tokens.push(lineTokens);
    }
  }
  return tokens;
}

function tokenizeWord(word: string): [Token, boolean] {
  if (word.startsWith("//")) {
    return [{ type: "symbol", value: word }, true];
  } else if (word.endsWith("%")) {
    return [{ type: "percent", value: parseFloat(word.slice(0, -1)) }, false];
  } else if (word.split("x").length == 2) {
    const [first, second] = word.split("x").map(parseFloat);
    return [{ type: "size", first, second }, false];
  } else if (word.startsWith("#")) {
    return [{ type: "color", value: word.slice(1) }, false];
  } else {
    return [{ type: "symbol", value: word }, false];
  }
}
