import { CharStreams, CommonTokenStream } from 'antlr4ts';
import { AbstractParseTreeVisitor } from 'antlr4ts/tree/AbstractParseTreeVisitor'
import { langLexer } from './parser/langLexer';
import { Component_declarationContext, ConnectionContext, langParser, PartContext, Part_bodyContext, ProgramContext } from './parser/langParser';
import { langVisitor, } from './parser/langVisitor'
import { UnresolvedComponent, UnresolvedPart, Connection, Part, PartBody, UnresolvedConnection, UnresolvedUse, Component } from './types';

export class ProgramVisitor extends AbstractParseTreeVisitor<UnresolvedComponent[]> implements langVisitor<UnresolvedComponent[]> {
    protected defaultResult(): UnresolvedComponent[] {
        throw new Error('Method not implemented.');
    }

    visitProgram(ctx: ProgramContext): UnresolvedComponent[] {
        const componentVisitor = new ComponentVisitor();
        const components = ctx.component_declaration().map(c => componentVisitor.visit(c))
        return components;
    }
}

class ComponentVisitor extends AbstractParseTreeVisitor<UnresolvedComponent> implements langVisitor<UnresolvedComponent> {
    protected defaultResult(): UnresolvedComponent {
        throw new Error('Method not implemented.');
    }

    visitComponent_declaration(ctx: Component_declarationContext): UnresolvedComponent {
        let name = ctx.component_name().text;
        let parts: UnresolvedPart[] = [];
        let uses: UnresolvedUse[] = [];
        let partVisitor = new PartVisitor();

        for (const part_or_use of ctx.component_body().part_or_use()) {
            if (part_or_use.part()) {
                let part = partVisitor.visit(part_or_use.part()!);
                parts.push(part);
            } else if (part_or_use.use()) {
                let use: UnresolvedUse = {
                    inputs: part_or_use.use()!.parameters().part_name().map(p => p.text),
                    outputs: part_or_use.use()!.outputs().part_name().map(o => o.text),
                    component: part_or_use.use()!.component_name().text
                }
                uses.push(use);
            }
        }

        return {
            name: name,
            parts: parts,
            uses: uses
        }
    }
}

class PartVisitor extends AbstractParseTreeVisitor<UnresolvedPart> implements langVisitor<UnresolvedPart> {
    protected defaultResult(): UnresolvedPart {
        throw new Error('Method not implemented.');
    }

    visitPart(ctx: PartContext): UnresolvedPart {
        let partBody = new PartBodyVisitor().visit(ctx.part_body());

        if (ctx.part_type().text == 'gear') {
            return {
                type: ctx.designator()?.text as 'input' | 'output' | undefined || 'other',
                part: {
                    kind: 'gear',
                    name: ctx.part_name().text,
                    teeth: partBody.options.get('teeth') as number,
                    connectedRight: partBody.connections,
                    state: partBody.options.get('state') as number | undefined || 0
                }
            }
        } else {
            return {
                type: ctx.designator()?.text as 'input' | 'output' | undefined || 'other', part: {
                    kind: 'rod',
                    name: ctx.part_name().text,
                    connectedRight: partBody.connections,
                    spring: partBody.options.get('spring') as 'pull' | 'push' | 'none' | undefined || 'none',
                    state: partBody.options.get('state') as 'push' | 'pull' | undefined || 'pull',
                }
            }
        }
    }
}

class PartBodyVisitor extends AbstractParseTreeVisitor<PartBody> implements langVisitor<PartBody> {
    protected defaultResult(): PartBody {
        throw new Error('Method not implemented.');
    }

    visitPart_body(ctx: Part_bodyContext): PartBody {
        let connections: UnresolvedConnection[] = [];
        let options = new Map();

        let connectionVisitor = new ConnectionVisitor();

        for (const body_item of ctx.part_body_item()) {
            if (body_item.option()) {
                let option = body_item.option()!;
                options.set(option.option_name().text, option.option_value().IDENTIFIER()?.text || parseInt(option.option_value().NUMBER()!.text, 10));
            } else {
                let connection = body_item.connection()!;
                connections.push(connectionVisitor.visit(connection));
            }
        }
        return {
            options: options,
            connections: connections
        }
    }
}

class ConnectionVisitor extends AbstractParseTreeVisitor<UnresolvedConnection> implements langVisitor<UnresolvedConnection> {
    protected defaultResult(): UnresolvedConnection {
        throw new Error('Method not implemented.');
    }

    visitConnection(ctx: ConnectionContext): UnresolvedConnection {
        let options: Map<string, string | number> = new Map();
        for (const option of ctx.connection_options().option()) {
            options.set(option.option_name().text, option.option_value().IDENTIFIER()?.text || parseInt(option.option_value().NUMBER()!.text, 10));
        }
        
        if (options.get('gearOffset') && typeof options.get('gearOffset') != 'number') {
            throw new Error('Invalid options! Gear offset must be a number! Line' + ctx.start.line);
        }
        else if (!(['push', 'pull', 'attach', undefined] as any[]).includes(options.get('rodAttachment'))) {
            throw new Error('Invalid options! rodAttachment must be push, pull or attach! Line' + ctx.start.line);
        }

        return {
            to: ctx.part_name()[1].text,
            gearOffset: options.get('gearOffset') as number | undefined,
            rodAttachment: options.get('rodAttachment') as 'push' | 'pull' | 'attach' | undefined,
        }
    }
}

/**
 * Resolves a single component to provide a Component
 */
export function resolveComponent(components: UnresolvedComponent[], name: string): Component  {
    const unresolvedComponent = components.find(c => c.name == name);
    if (!unresolvedComponent) {
        throw new Error("Component not found: " + name);   
    }

    let parts: Part[] = [];
    let inputs: Part[] = [];
    let outputs: Part[] = [];

    for (const part of unresolvedComponent.parts) {
        let resolvedPart = resolvePart(part, parts, unresolvedComponent.parts);
        parts.push(resolvedPart);
        if (part.type == 'input') {
            inputs.push(resolvedPart);
        } else if (part.type == 'output') {
            outputs.push(resolvedPart);
        }
    }

    for (const use of unresolvedComponent.uses) {
        let usedComponent = resolveComponent(components, use.component);
        for (let i = 0; i < use.inputs.length; i++) {
            const part = parts.find(p => p.name == use.inputs[i]);
            if (!part) { throw new Error("Part not found: " + use.inputs[i]); }

            part.connectedRight.push({
                kind: 'use',
                part: usedComponent,
                parameterIndex: i
            })
        }

        usedComponent.output_map = [];
        for (let i = 0; i < use.outputs.length; i++) {
            const part = parts.find(p => p.name == use.outputs[i]);
            if (!part) { throw new Error("Part not found: " + use.outputs[i]); }

            usedComponent.output_map.push(part);
        }
    }

    return {
        kind: 'component',
        name: unresolvedComponent.name,
        inputs: inputs,
        outputs: outputs,
    }
}

function resolvePart(part: UnresolvedPart, resolvedParts: Part[], unresolvedParts: UnresolvedPart[]): Part {
    let alreadyResolved = resolvedParts.find(p => p.name == part.part.name);
    if (alreadyResolved) {
        return alreadyResolved;
    }

    if (part.part.kind == 'gear') {
        let connections: Connection[] = [];
        for (const unresolvedConnection of part.part.connectedRight) {
            let otherPart: Part | undefined = resolvedParts.find(p => p.name == unresolvedConnection.to);
            if (!otherPart) {
                let unresolvedOtherPart = unresolvedParts.find(p => p.part.name == unresolvedConnection.to);
                if (!unresolvedOtherPart) {
                    throw new Error("Part not found: " + unresolvedConnection.to);
                }
                otherPart = resolvePart(unresolvedOtherPart, resolvedParts, unresolvedParts);
                resolvedParts.push(otherPart);
            }
            let kind: "rod-rod" | "gear-rod" | "gear-gear" = otherPart.kind == 'rod' ? 'gear-rod' : 'gear-gear';
            let connection: Connection = {
                kind: kind, 
                part: otherPart,
                // @ts-ignore
                gearOffset: unresolvedConnection.gearOffset,
                // @ts-ignore
                rodAttachment: unresolvedConnection.rodAttachment
            }

            connections.push(connection);
        }

        let resolvedPart: Part = {
            kind: 'gear',
            name: part.part.name,
            teeth: part.part.teeth,
            connectedRight: connections,
            state: 0
        };

        return resolvedPart;
    } else {
        let connections: Connection[] = [];
        for (const unresolvedConnection of part.part.connectedRight) {
            let otherPart: Part | undefined = resolvedParts.find(p => p.name == unresolvedConnection.to);
            if (!otherPart) {
                let unresolvedOtherPart = unresolvedParts.find(p => p.part.name == unresolvedConnection.to);
                if (!unresolvedOtherPart) {
                    throw new Error("Part not found: " + unresolvedConnection.to);
                }
                otherPart = resolvePart(unresolvedOtherPart, resolvedParts, unresolvedParts);
                resolvedParts.push(otherPart);
            }

            let kind: "rod-rod" | "gear-rod" | "gear-gear" = otherPart.kind == 'rod' ? 'rod-rod' : 'gear-rod';
            let connection: Connection = {
                kind: kind, 
                part: otherPart,
                // @ts-ignore
                gearOffset: unresolvedConnection.gearOffset,
                // @ts-ignore
                rodAttachment: unresolvedConnection.rodAttachment
            }

            connections.push(connection);
        }

        let resolvedPart: Part = {
            kind: 'rod',
            name: part.part.name,
            spring: part.part.spring,
            connectedRight: connections,
            state: 'pull'
        };

        return resolvedPart;
    }
}