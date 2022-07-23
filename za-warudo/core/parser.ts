import { Token } from "./tokenizer";

interface World {
  regions: Region[];
}

interface Region {
  name: string;
  percent: number;
  maxSize: [number, number];
  subRegions: SubRegion[];
}

interface SubRegion {
  name: string;
  color: string;
  percent: number;
}

class ParseError {
  constructor(public message: string, public line: number) {}
}

// Either returns a valid world or an error as a string.
export function parser(tokens: Token[][]): [World, ParseError | null] {
  let worldInitialized = false;
  const world: World = { regions: [] };

  if (tokens.length == 0) {
    return [world, new ParseError("No input", 0)];
  }

  tokens.forEach((line, lineNumber) => {
    if (line.length == 0) {
      return [world, new ParseError("Empty line?", lineNumber)];
    }
    // TODO: rest
  });

  return [world, null];
}
