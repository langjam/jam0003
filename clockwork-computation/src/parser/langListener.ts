// Generated from lang.g4 by ANTLR 4.9.0-SNAPSHOT


import { ParseTreeListener } from "antlr4ts/tree/ParseTreeListener";

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
 * This interface defines a complete listener for a parse tree produced by
 * `langParser`.
 */
export interface langListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by `langParser.program`.
	 * @param ctx the parse tree
	 */
	enterProgram?: (ctx: ProgramContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.program`.
	 * @param ctx the parse tree
	 */
	exitProgram?: (ctx: ProgramContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.component_declaration`.
	 * @param ctx the parse tree
	 */
	enterComponent_declaration?: (ctx: Component_declarationContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.component_declaration`.
	 * @param ctx the parse tree
	 */
	exitComponent_declaration?: (ctx: Component_declarationContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.component_name`.
	 * @param ctx the parse tree
	 */
	enterComponent_name?: (ctx: Component_nameContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.component_name`.
	 * @param ctx the parse tree
	 */
	exitComponent_name?: (ctx: Component_nameContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.component_body`.
	 * @param ctx the parse tree
	 */
	enterComponent_body?: (ctx: Component_bodyContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.component_body`.
	 * @param ctx the parse tree
	 */
	exitComponent_body?: (ctx: Component_bodyContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.part_or_use`.
	 * @param ctx the parse tree
	 */
	enterPart_or_use?: (ctx: Part_or_useContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.part_or_use`.
	 * @param ctx the parse tree
	 */
	exitPart_or_use?: (ctx: Part_or_useContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.part`.
	 * @param ctx the parse tree
	 */
	enterPart?: (ctx: PartContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.part`.
	 * @param ctx the parse tree
	 */
	exitPart?: (ctx: PartContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.part_name`.
	 * @param ctx the parse tree
	 */
	enterPart_name?: (ctx: Part_nameContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.part_name`.
	 * @param ctx the parse tree
	 */
	exitPart_name?: (ctx: Part_nameContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.designator`.
	 * @param ctx the parse tree
	 */
	enterDesignator?: (ctx: DesignatorContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.designator`.
	 * @param ctx the parse tree
	 */
	exitDesignator?: (ctx: DesignatorContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.part_type`.
	 * @param ctx the parse tree
	 */
	enterPart_type?: (ctx: Part_typeContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.part_type`.
	 * @param ctx the parse tree
	 */
	exitPart_type?: (ctx: Part_typeContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.part_body`.
	 * @param ctx the parse tree
	 */
	enterPart_body?: (ctx: Part_bodyContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.part_body`.
	 * @param ctx the parse tree
	 */
	exitPart_body?: (ctx: Part_bodyContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.part_body_item`.
	 * @param ctx the parse tree
	 */
	enterPart_body_item?: (ctx: Part_body_itemContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.part_body_item`.
	 * @param ctx the parse tree
	 */
	exitPart_body_item?: (ctx: Part_body_itemContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.option`.
	 * @param ctx the parse tree
	 */
	enterOption?: (ctx: OptionContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.option`.
	 * @param ctx the parse tree
	 */
	exitOption?: (ctx: OptionContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.option_name`.
	 * @param ctx the parse tree
	 */
	enterOption_name?: (ctx: Option_nameContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.option_name`.
	 * @param ctx the parse tree
	 */
	exitOption_name?: (ctx: Option_nameContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.option_value`.
	 * @param ctx the parse tree
	 */
	enterOption_value?: (ctx: Option_valueContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.option_value`.
	 * @param ctx the parse tree
	 */
	exitOption_value?: (ctx: Option_valueContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.connection`.
	 * @param ctx the parse tree
	 */
	enterConnection?: (ctx: ConnectionContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.connection`.
	 * @param ctx the parse tree
	 */
	exitConnection?: (ctx: ConnectionContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.connection_options`.
	 * @param ctx the parse tree
	 */
	enterConnection_options?: (ctx: Connection_optionsContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.connection_options`.
	 * @param ctx the parse tree
	 */
	exitConnection_options?: (ctx: Connection_optionsContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.use`.
	 * @param ctx the parse tree
	 */
	enterUse?: (ctx: UseContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.use`.
	 * @param ctx the parse tree
	 */
	exitUse?: (ctx: UseContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.parameters`.
	 * @param ctx the parse tree
	 */
	enterParameters?: (ctx: ParametersContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.parameters`.
	 * @param ctx the parse tree
	 */
	exitParameters?: (ctx: ParametersContext) => void;

	/**
	 * Enter a parse tree produced by `langParser.outputs`.
	 * @param ctx the parse tree
	 */
	enterOutputs?: (ctx: OutputsContext) => void;
	/**
	 * Exit a parse tree produced by `langParser.outputs`.
	 * @param ctx the parse tree
	 */
	exitOutputs?: (ctx: OutputsContext) => void;
}

