// Generated from lang.g4 by ANTLR 4.9.0-SNAPSHOT


import { ParseTreeVisitor } from "antlr4ts/tree/ParseTreeVisitor";

import { ProgramContext } from "./langParser";
import { Component_declarationContext } from "./langParser";
import { Component_nameContext } from "./langParser";
import { Component_bodyContext } from "./langParser";
import { Part_or_useContext } from "./langParser";
import { PartContext } from "./langParser";
import { Part_nameContext } from "./langParser";
import { DesignatorContext } from "./langParser";
import { Part_typeContext } from "./langParser";
import { Part_bodyContext } from "./langParser";
import { Part_body_itemContext } from "./langParser";
import { OptionContext } from "./langParser";
import { Option_nameContext } from "./langParser";
import { Option_valueContext } from "./langParser";
import { ConnectionContext } from "./langParser";
import { Connection_optionsContext } from "./langParser";
import { UseContext } from "./langParser";
import { ParametersContext } from "./langParser";
import { OutputsContext } from "./langParser";


/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by `langParser`.
 *
 * @param <Result> The return type of the visit operation. Use `void` for
 * operations with no return type.
 */
export interface langVisitor<Result> extends ParseTreeVisitor<Result> {
	/**
	 * Visit a parse tree produced by `langParser.program`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitProgram?: (ctx: ProgramContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.component_declaration`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitComponent_declaration?: (ctx: Component_declarationContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.component_name`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitComponent_name?: (ctx: Component_nameContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.component_body`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitComponent_body?: (ctx: Component_bodyContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.part_or_use`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitPart_or_use?: (ctx: Part_or_useContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.part`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitPart?: (ctx: PartContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.part_name`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitPart_name?: (ctx: Part_nameContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.designator`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitDesignator?: (ctx: DesignatorContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.part_type`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitPart_type?: (ctx: Part_typeContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.part_body`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitPart_body?: (ctx: Part_bodyContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.part_body_item`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitPart_body_item?: (ctx: Part_body_itemContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.option`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitOption?: (ctx: OptionContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.option_name`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitOption_name?: (ctx: Option_nameContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.option_value`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitOption_value?: (ctx: Option_valueContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.connection`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitConnection?: (ctx: ConnectionContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.connection_options`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitConnection_options?: (ctx: Connection_optionsContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.use`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitUse?: (ctx: UseContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.parameters`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitParameters?: (ctx: ParametersContext) => Result;

	/**
	 * Visit a parse tree produced by `langParser.outputs`.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	visitOutputs?: (ctx: OutputsContext) => Result;
}

