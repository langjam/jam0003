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
  constructor(public message: string, public line: number) { }
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
    if (line[0].type != 'symbol') {
      return [world, new ParseError("Doesn't start with a symbol", lineNumber)];
    }
    if (line[0].value == 'region') {
      if (line.length != 2) {
        return [world, new ParseError("`region` expects 1 argument", lineNumber)];
      }
      if (line[1].type != 'symbol') {
        return [world, new ParseError("`region` expects a region name as argument", lineNumber)];
      }
      var region: Region = {
        name: line[1].value,
        percent: -1,
        maxSize: [-1, -1],
        subRegions: []
      };
      world.regions.push(region);
    } else {
      switch (line.length) {
        case 2:
          if (line[0].type != 'symbol') {
            return [world, new ParseError("Expected symbol got something else", lineNumber)];
          }
          // TODO: iterate through world region names to check if this exists
          break;
        case 3: break;
        default: return [world, new ParseError("Unexpected number of arguments", lineNumber)];
      }
    }
  }
  );

  return [world, null];
}
