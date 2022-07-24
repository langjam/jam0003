import { Injectable } from '@angular/core';

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

export interface World {
  regions: Region[];
  legends: Legend[];
}

interface Legend {
  name: string;
  color: string;
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

@Injectable({
  providedIn: 'root'
})
export class CompilerService {

  constructor() { }

  tokenize(input: string): Token[][] {
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

  // Either returns a valid world or an error as a string.
  parse(tokens: Token[][]): [World, ParseError | null] {
    let worldInitialized = false;
    const world: World = { regions: [], legends: [] };

    if (tokens.length == 0) {
      return [world, new ParseError("No input", 0)];
    }

    for (const [lineNumber, line] of tokens.entries()) {
      if (line.length == 0) {
        return [world, new ParseError("Empty line?", lineNumber)];
      }
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
      } else if (line[0].value == 'world') {
        worldInitialized = true;
      } else if (line[0].value == 'legend') {
        if (line.length != 3) {
          return [world, new ParseError("`legend` expects 2 arguments", lineNumber)];
        }
        if (line[1].type != 'symbol') {
          return [world, new ParseError("`legend` expects a legend name as argument", lineNumber)];
        }
        if (line[2].type != 'color') {
          return [world, new ParseError("`legend` expects a color as argument", lineNumber)];
        }
        var legend: Legend = {
          name: line[1].value,
          color: line[2].value
        };
        world.legends.push(legend);
      } else {
        switch (line.length) {
          case 2:
            const first = line[0];
            if (first.type != 'symbol') {
              return [world, new ParseError("Expected symbol got something else", lineNumber)];
            }
            const found = world.regions.find(r => r.name == first.value);
            if (!found) {
              return [world, new ParseError(`Region not found: ${first.value}`, lineNumber)];
            }
            const second = line[1];
            switch (second.type) {
              case "percent":
                found.percent = second.value;
                break;
              case "size":
                found.maxSize = [second.first, second.second];
                break;
              default: return [world, new ParseError("Expected percent or size", lineNumber)];
            }
            break;
          case 3:
            const forst = line[0];
            const secind = line[1];
            const third = line[2];
            if (forst.type != 'symbol') {
              return [world, new ParseError("Expected symbol got something else", lineNumber)];
            }
            const foundRegion = world.regions.find(r => r.name == forst.value);
            if (!foundRegion) {
              return [world, new ParseError(`Region not found: ${forst.value}`, lineNumber)];
            }
            if (secind.type != 'symbol') {
              return [world, new ParseError("Expected symbol in 2nd place", lineNumber)];
            }
            const foundLegend = world.legends.find(l => l.name == secind.value);
            if (!foundLegend) {
              return [world, new ParseError(`Legend not found: ${secind.value}`, lineNumber)];
            }
            if (third.type != 'percent') {
              return [world, new ParseError("Expected percent in 3rd place", lineNumber)];
            }
            const subRegion: SubRegion = {
              name: secind.value,
              color: foundLegend.color,
              percent: third.value
            };
            foundRegion.subRegions.push(subRegion);
            break;
          default: return [world, new ParseError("Unexpected number of arguments", lineNumber)];
        }
      }
    }

    return [world, null];
  }
}
