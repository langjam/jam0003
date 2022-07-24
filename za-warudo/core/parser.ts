import { Token } from "./tokenizer";

interface World {
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

// Either returns a valid world or an error as a string.
export function parser(tokens: Token[][]): [World, ParseError | null] {
  const world: World = { regions: [], legends: [] };

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
      // TODO: legend
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
  );

  return [world, null];
}
