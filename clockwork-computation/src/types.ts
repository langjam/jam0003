export type Component = {
    kind: "component";
    name: string;
    inputs: Part[];
    outputs: Part[];
    output_map?: Part[]; // For used components, apply the outputs to these parts.  
}

export type UnresolvedComponent = {
    name: string;
    parts: UnresolvedPart[];
    uses: UnresolvedUse[];
}

export type UnresolvedUse  ={
    inputs: string[];
    outputs: string[];
    component: string;
}

export type UnresolvedPart = {
    type: "input" | "output" | "other";
    part: UnresolvedGear | UnresolvedRod;
}

export type PartBody = {
    options: Map<string, string| number>;
    connections: UnresolvedConnection[];
}

export type UnresolvedConnection = {
    to: string;
    gearOffset?: number;
    rodAttachment?: "push" | "pull" | "attach";
}

export type UnresolvedGear = {
    kind: "gear";
    name: string;
    teeth: number;
    connectedRight: UnresolvedConnection[];
    state: number; // number of teeth turned
}


export type UnresolvedRod = {
    name: string;
    kind: "rod";
    connectedRight: UnresolvedConnection[];
    spring: 'pull' | 'push' | 'none';
    state: 'push' | 'pull';
}

export type Part = Gear | Rod;

export type Connection = RodRodConnection | GearRodConnection | GearGearConnection | UseConnection;

export type RodRodConnection = {
    kind: "rod-rod";
    part: Part;
    rodAttachment: "push" | "pull" | "attach";
}

export type GearRodConnection = {
    kind: "gear-rod";
    part: Part;
    gearOffset: number;
}

export type GearGearConnection = {
    kind: "gear-gear";
    part: Part;
}

export type UseConnection = {
    kind: "use";
    part: Component;
    parameterIndex: number;
}

export type Gear = {
    kind: "gear";
    name: string;
    teeth: number;
    connectedRight: Connection[];
    state: number; // number of teeth turned
}

export type Rod = {
    name: string;
    kind: "rod";
    connectedRight: Connection[];
    spring: 'pull' | 'push' | 'none';
    state: 'push' | 'pull';
}

export enum PullDirection {
    Left,
    Right
}

