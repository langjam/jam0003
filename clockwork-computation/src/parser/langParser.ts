// Generated from lang.g4 by ANTLR 4.9.0-SNAPSHOT


import { ATN } from "antlr4ts/atn/ATN";
import { ATNDeserializer } from "antlr4ts/atn/ATNDeserializer";
import { FailedPredicateException } from "antlr4ts/FailedPredicateException";
import { NotNull } from "antlr4ts/Decorators";
import { NoViableAltException } from "antlr4ts/NoViableAltException";
import { Override } from "antlr4ts/Decorators";
import { Parser } from "antlr4ts/Parser";
import { ParserRuleContext } from "antlr4ts/ParserRuleContext";
import { ParserATNSimulator } from "antlr4ts/atn/ParserATNSimulator";
import { ParseTreeListener } from "antlr4ts/tree/ParseTreeListener";
import { ParseTreeVisitor } from "antlr4ts/tree/ParseTreeVisitor";
import { RecognitionException } from "antlr4ts/RecognitionException";
import { RuleContext } from "antlr4ts/RuleContext";
//import { RuleVersion } from "antlr4ts/RuleVersion";
import { TerminalNode } from "antlr4ts/tree/TerminalNode";
import { Token } from "antlr4ts/Token";
import { TokenStream } from "antlr4ts/TokenStream";
import { Vocabulary } from "antlr4ts/Vocabulary";
import { VocabularyImpl } from "antlr4ts/VocabularyImpl";

import * as Utils from "antlr4ts/misc/Utils";

import { langListener } from "./langListener";
import { langVisitor } from "./langVisitor";


export class langParser extends Parser {
	public static readonly T__0 = 1;
	public static readonly T__1 = 2;
	public static readonly T__2 = 3;
	public static readonly T__3 = 4;
	public static readonly T__4 = 5;
	public static readonly T__5 = 6;
	public static readonly T__6 = 7;
	public static readonly T__7 = 8;
	public static readonly T__8 = 9;
	public static readonly T__9 = 10;
	public static readonly T__10 = 11;
	public static readonly T__11 = 12;
	public static readonly T__12 = 13;
	public static readonly NUMBER = 14;
	public static readonly IDENTIFIER = 15;
	public static readonly WS = 16;
	public static readonly COMMENT = 17;
	public static readonly RULE_program = 0;
	public static readonly RULE_component_declaration = 1;
	public static readonly RULE_component_name = 2;
	public static readonly RULE_component_body = 3;
	public static readonly RULE_part_or_use = 4;
	public static readonly RULE_part = 5;
	public static readonly RULE_part_name = 6;
	public static readonly RULE_designator = 7;
	public static readonly RULE_part_type = 8;
	public static readonly RULE_part_body = 9;
	public static readonly RULE_part_body_item = 10;
	public static readonly RULE_option = 11;
	public static readonly RULE_option_name = 12;
	public static readonly RULE_option_value = 13;
	public static readonly RULE_connection = 14;
	public static readonly RULE_connection_options = 15;
	public static readonly RULE_use = 16;
	public static readonly RULE_parameters = 17;
	public static readonly RULE_outputs = 18;
	// tslint:disable:no-trailing-whitespace
	public static readonly ruleNames: string[] = [
		"program", "component_declaration", "component_name", "component_body", 
		"part_or_use", "part", "part_name", "designator", "part_type", "part_body", 
		"part_body_item", "option", "option_name", "option_value", "connection", 
		"connection_options", "use", "parameters", "outputs",
	];

	private static readonly _LITERAL_NAMES: Array<string | undefined> = [
		undefined, "'component'", "'{'", "'}'", "':'", "'input'", "'output'", 
		"'gear'", "'rod'", "'->'", "'use'", "'('", "')'", "','",
	];
	private static readonly _SYMBOLIC_NAMES: Array<string | undefined> = [
		undefined, undefined, undefined, undefined, undefined, undefined, undefined, 
		undefined, undefined, undefined, undefined, undefined, undefined, undefined, 
		"NUMBER", "IDENTIFIER", "WS", "COMMENT",
	];
	public static readonly VOCABULARY: Vocabulary = new VocabularyImpl(langParser._LITERAL_NAMES, langParser._SYMBOLIC_NAMES, []);

	// @Override
	// @NotNull
	public get vocabulary(): Vocabulary {
		return langParser.VOCABULARY;
	}
	// tslint:enable:no-trailing-whitespace

	// @Override
	public get grammarFileName(): string { return "lang.g4"; }

	// @Override
	public get ruleNames(): string[] { return langParser.ruleNames; }

	// @Override
	public get serializedATN(): string { return langParser._serializedATN; }

	protected createFailedPredicateException(predicate?: string, message?: string): FailedPredicateException {
		return new FailedPredicateException(this, predicate, message);
	}

	constructor(input: TokenStream) {
		super(input);
		this._interp = new ParserATNSimulator(langParser._ATN, this);
	}
	// @RuleVersion(0)
	public program(): ProgramContext {
		let _localctx: ProgramContext = new ProgramContext(this._ctx, this.state);
		this.enterRule(_localctx, 0, langParser.RULE_program);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 39;
			this._errHandler.sync(this);
			_la = this._input.LA(1);
			do {
				{
				{
				this.state = 38;
				this.component_declaration();
				}
				}
				this.state = 41;
				this._errHandler.sync(this);
				_la = this._input.LA(1);
			} while (_la === langParser.T__0);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public component_declaration(): Component_declarationContext {
		let _localctx: Component_declarationContext = new Component_declarationContext(this._ctx, this.state);
		this.enterRule(_localctx, 2, langParser.RULE_component_declaration);
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 43;
			this.match(langParser.T__0);
			this.state = 44;
			this.component_name();
			this.state = 45;
			this.match(langParser.T__1);
			this.state = 46;
			this.component_body();
			this.state = 47;
			this.match(langParser.T__2);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public component_name(): Component_nameContext {
		let _localctx: Component_nameContext = new Component_nameContext(this._ctx, this.state);
		this.enterRule(_localctx, 4, langParser.RULE_component_name);
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 49;
			this.match(langParser.IDENTIFIER);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public component_body(): Component_bodyContext {
		let _localctx: Component_bodyContext = new Component_bodyContext(this._ctx, this.state);
		this.enterRule(_localctx, 6, langParser.RULE_component_body);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 54;
			this._errHandler.sync(this);
			_la = this._input.LA(1);
			while ((((_la) & ~0x1F) === 0 && ((1 << _la) & ((1 << langParser.T__4) | (1 << langParser.T__5) | (1 << langParser.T__9) | (1 << langParser.IDENTIFIER))) !== 0)) {
				{
				{
				this.state = 51;
				this.part_or_use();
				}
				}
				this.state = 56;
				this._errHandler.sync(this);
				_la = this._input.LA(1);
			}
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public part_or_use(): Part_or_useContext {
		let _localctx: Part_or_useContext = new Part_or_useContext(this._ctx, this.state);
		this.enterRule(_localctx, 8, langParser.RULE_part_or_use);
		try {
			this.state = 59;
			this._errHandler.sync(this);
			switch (this._input.LA(1)) {
			case langParser.T__4:
			case langParser.T__5:
			case langParser.IDENTIFIER:
				this.enterOuterAlt(_localctx, 1);
				{
				this.state = 57;
				this.part();
				}
				break;
			case langParser.T__9:
				this.enterOuterAlt(_localctx, 2);
				{
				this.state = 58;
				this.use();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public part(): PartContext {
		let _localctx: PartContext = new PartContext(this._ctx, this.state);
		this.enterRule(_localctx, 10, langParser.RULE_part);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 62;
			this._errHandler.sync(this);
			_la = this._input.LA(1);
			if (_la === langParser.T__4 || _la === langParser.T__5) {
				{
				this.state = 61;
				this.designator();
				}
			}

			this.state = 64;
			this.part_name();
			this.state = 65;
			this.match(langParser.T__3);
			this.state = 66;
			this.part_type();
			this.state = 67;
			this.match(langParser.T__1);
			this.state = 68;
			this.part_body();
			this.state = 69;
			this.match(langParser.T__2);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public part_name(): Part_nameContext {
		let _localctx: Part_nameContext = new Part_nameContext(this._ctx, this.state);
		this.enterRule(_localctx, 12, langParser.RULE_part_name);
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 71;
			this.match(langParser.IDENTIFIER);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public designator(): DesignatorContext {
		let _localctx: DesignatorContext = new DesignatorContext(this._ctx, this.state);
		this.enterRule(_localctx, 14, langParser.RULE_designator);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 73;
			_la = this._input.LA(1);
			if (!(_la === langParser.T__4 || _la === langParser.T__5)) {
			this._errHandler.recoverInline(this);
			} else {
				if (this._input.LA(1) === Token.EOF) {
					this.matchedEOF = true;
				}

				this._errHandler.reportMatch(this);
				this.consume();
			}
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public part_type(): Part_typeContext {
		let _localctx: Part_typeContext = new Part_typeContext(this._ctx, this.state);
		this.enterRule(_localctx, 16, langParser.RULE_part_type);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 75;
			_la = this._input.LA(1);
			if (!(_la === langParser.T__6 || _la === langParser.T__7)) {
			this._errHandler.recoverInline(this);
			} else {
				if (this._input.LA(1) === Token.EOF) {
					this.matchedEOF = true;
				}

				this._errHandler.reportMatch(this);
				this.consume();
			}
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public part_body(): Part_bodyContext {
		let _localctx: Part_bodyContext = new Part_bodyContext(this._ctx, this.state);
		this.enterRule(_localctx, 18, langParser.RULE_part_body);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 80;
			this._errHandler.sync(this);
			_la = this._input.LA(1);
			while (_la === langParser.IDENTIFIER) {
				{
				{
				this.state = 77;
				this.part_body_item();
				}
				}
				this.state = 82;
				this._errHandler.sync(this);
				_la = this._input.LA(1);
			}
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public part_body_item(): Part_body_itemContext {
		let _localctx: Part_body_itemContext = new Part_body_itemContext(this._ctx, this.state);
		this.enterRule(_localctx, 20, langParser.RULE_part_body_item);
		try {
			this.state = 85;
			this._errHandler.sync(this);
			switch ( this.interpreter.adaptivePredict(this._input, 5, this._ctx) ) {
			case 1:
				this.enterOuterAlt(_localctx, 1);
				{
				this.state = 83;
				this.option();
				}
				break;

			case 2:
				this.enterOuterAlt(_localctx, 2);
				{
				this.state = 84;
				this.connection();
				}
				break;
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public option(): OptionContext {
		let _localctx: OptionContext = new OptionContext(this._ctx, this.state);
		this.enterRule(_localctx, 22, langParser.RULE_option);
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 87;
			this.option_name();
			this.state = 88;
			this.match(langParser.T__3);
			this.state = 89;
			this.option_value();
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public option_name(): Option_nameContext {
		let _localctx: Option_nameContext = new Option_nameContext(this._ctx, this.state);
		this.enterRule(_localctx, 24, langParser.RULE_option_name);
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 91;
			this.match(langParser.IDENTIFIER);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public option_value(): Option_valueContext {
		let _localctx: Option_valueContext = new Option_valueContext(this._ctx, this.state);
		this.enterRule(_localctx, 26, langParser.RULE_option_value);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 93;
			_la = this._input.LA(1);
			if (!(_la === langParser.NUMBER || _la === langParser.IDENTIFIER)) {
			this._errHandler.recoverInline(this);
			} else {
				if (this._input.LA(1) === Token.EOF) {
					this.matchedEOF = true;
				}

				this._errHandler.reportMatch(this);
				this.consume();
			}
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public connection(): ConnectionContext {
		let _localctx: ConnectionContext = new ConnectionContext(this._ctx, this.state);
		this.enterRule(_localctx, 28, langParser.RULE_connection);
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 95;
			this.part_name();
			this.state = 96;
			this.match(langParser.T__8);
			this.state = 97;
			this.part_name();
			this.state = 98;
			this.match(langParser.T__1);
			this.state = 99;
			this.connection_options();
			this.state = 100;
			this.match(langParser.T__2);
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public connection_options(): Connection_optionsContext {
		let _localctx: Connection_optionsContext = new Connection_optionsContext(this._ctx, this.state);
		this.enterRule(_localctx, 30, langParser.RULE_connection_options);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 105;
			this._errHandler.sync(this);
			_la = this._input.LA(1);
			while (_la === langParser.IDENTIFIER) {
				{
				{
				this.state = 102;
				this.option();
				}
				}
				this.state = 107;
				this._errHandler.sync(this);
				_la = this._input.LA(1);
			}
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public use(): UseContext {
		let _localctx: UseContext = new UseContext(this._ctx, this.state);
		this.enterRule(_localctx, 32, langParser.RULE_use);
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 108;
			this.match(langParser.T__9);
			this.state = 109;
			this.component_name();
			this.state = 110;
			this.match(langParser.T__10);
			this.state = 111;
			this.parameters();
			this.state = 112;
			this.match(langParser.T__11);
			this.state = 113;
			this.match(langParser.T__8);
			this.state = 114;
			this.outputs();
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public parameters(): ParametersContext {
		let _localctx: ParametersContext = new ParametersContext(this._ctx, this.state);
		this.enterRule(_localctx, 34, langParser.RULE_parameters);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			this.state = 124;
			this._errHandler.sync(this);
			_la = this._input.LA(1);
			if (_la === langParser.IDENTIFIER) {
				{
				this.state = 116;
				this.part_name();
				this.state = 121;
				this._errHandler.sync(this);
				_la = this._input.LA(1);
				while (_la === langParser.T__12) {
					{
					{
					this.state = 117;
					this.match(langParser.T__12);
					this.state = 118;
					this.part_name();
					}
					}
					this.state = 123;
					this._errHandler.sync(this);
					_la = this._input.LA(1);
				}
				}
			}

			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}
	// @RuleVersion(0)
	public outputs(): OutputsContext {
		let _localctx: OutputsContext = new OutputsContext(this._ctx, this.state);
		this.enterRule(_localctx, 36, langParser.RULE_outputs);
		let _la: number;
		try {
			this.enterOuterAlt(_localctx, 1);
			{
			{
			this.state = 126;
			this.part_name();
			this.state = 131;
			this._errHandler.sync(this);
			_la = this._input.LA(1);
			while (_la === langParser.T__12) {
				{
				{
				this.state = 127;
				this.match(langParser.T__12);
				this.state = 128;
				this.part_name();
				}
				}
				this.state = 133;
				this._errHandler.sync(this);
				_la = this._input.LA(1);
			}
			}
			}
		}
		catch (re) {
			if (re instanceof RecognitionException) {
				_localctx.exception = re;
				this._errHandler.reportError(this, re);
				this._errHandler.recover(this, re);
			} else {
				throw re;
			}
		}
		finally {
			this.exitRule();
		}
		return _localctx;
	}

	public static readonly _serializedATN: string =
		"\x03\uC91D\uCABA\u058D\uAFBA\u4F53\u0607\uEA8B\uC241\x03\x13\x89\x04\x02" +
		"\t\x02\x04\x03\t\x03\x04\x04\t\x04\x04\x05\t\x05\x04\x06\t\x06\x04\x07" +
		"\t\x07\x04\b\t\b\x04\t\t\t\x04\n\t\n\x04\v\t\v\x04\f\t\f\x04\r\t\r\x04" +
		"\x0E\t\x0E\x04\x0F\t\x0F\x04\x10\t\x10\x04\x11\t\x11\x04\x12\t\x12\x04" +
		"\x13\t\x13\x04\x14\t\x14\x03\x02\x06\x02*\n\x02\r\x02\x0E\x02+\x03\x03" +
		"\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x05\x07\x05" +
		"7\n\x05\f\x05\x0E\x05:\v\x05\x03\x06\x03\x06\x05\x06>\n\x06\x03\x07\x05" +
		"\x07A\n\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03" +
		"\b\x03\b\x03\t\x03\t\x03\n\x03\n\x03\v\x07\vQ\n\v\f\v\x0E\vT\v\v\x03\f" +
		"\x03\f\x05\fX\n\f\x03\r\x03\r\x03\r\x03\r\x03\x0E\x03\x0E\x03\x0F\x03" +
		"\x0F\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x11\x07" +
		"\x11j\n\x11\f\x11\x0E\x11m\v\x11\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12" +
		"\x03\x12\x03\x12\x03\x12\x03\x13\x03\x13\x03\x13\x07\x13z\n\x13\f\x13" +
		"\x0E\x13}\v\x13\x05\x13\x7F\n\x13\x03\x14\x03\x14\x03\x14\x07\x14\x84" +
		"\n\x14\f\x14\x0E\x14\x87\v\x14\x03\x14\x02\x02\x02\x15\x02\x02\x04\x02" +
		"\x06\x02\b\x02\n\x02\f\x02\x0E\x02\x10\x02\x12\x02\x14\x02\x16\x02\x18" +
		"\x02\x1A\x02\x1C\x02\x1E\x02 \x02\"\x02$\x02&\x02\x02\x05\x03\x02\x07" +
		"\b\x03\x02\t\n\x03\x02\x10\x11\x02\x7F\x02)\x03\x02\x02\x02\x04-\x03\x02" +
		"\x02\x02\x063\x03\x02\x02\x02\b8\x03\x02\x02\x02\n=\x03\x02\x02\x02\f" +
		"@\x03\x02\x02\x02\x0EI\x03\x02\x02\x02\x10K\x03\x02\x02\x02\x12M\x03\x02" +
		"\x02\x02\x14R\x03\x02\x02\x02\x16W\x03\x02\x02\x02\x18Y\x03\x02\x02\x02" +
		"\x1A]\x03\x02\x02\x02\x1C_\x03\x02\x02\x02\x1Ea\x03\x02\x02\x02 k\x03" +
		"\x02\x02\x02\"n\x03\x02\x02\x02$~\x03\x02\x02\x02&\x80\x03\x02\x02\x02" +
		"(*\x05\x04\x03\x02)(\x03\x02\x02\x02*+\x03\x02\x02\x02+)\x03\x02\x02\x02" +
		"+,\x03\x02\x02\x02,\x03\x03\x02\x02\x02-.\x07\x03\x02\x02./\x05\x06\x04" +
		"\x02/0\x07\x04\x02\x0201\x05\b\x05\x0212\x07\x05\x02\x022\x05\x03\x02" +
		"\x02\x0234\x07\x11\x02\x024\x07\x03\x02\x02\x0257\x05\n\x06\x0265\x03" +
		"\x02\x02\x027:\x03\x02\x02\x0286\x03\x02\x02\x0289\x03\x02\x02\x029\t" +
		"\x03\x02\x02\x02:8\x03\x02\x02\x02;>\x05\f\x07\x02<>\x05\"\x12\x02=;\x03" +
		"\x02\x02\x02=<\x03\x02\x02\x02>\v\x03\x02\x02\x02?A\x05\x10\t\x02@?\x03" +
		"\x02\x02\x02@A\x03\x02\x02\x02AB\x03\x02\x02\x02BC\x05\x0E\b\x02CD\x07" +
		"\x06\x02\x02DE\x05\x12\n\x02EF\x07\x04\x02\x02FG\x05\x14\v\x02GH\x07\x05" +
		"\x02\x02H\r\x03\x02\x02\x02IJ\x07\x11\x02\x02J\x0F\x03\x02\x02\x02KL\t" +
		"\x02\x02\x02L\x11\x03\x02\x02\x02MN\t\x03\x02\x02N\x13\x03\x02\x02\x02" +
		"OQ\x05\x16\f\x02PO\x03\x02\x02\x02QT\x03\x02\x02\x02RP\x03\x02\x02\x02" +
		"RS\x03\x02\x02\x02S\x15\x03\x02\x02\x02TR\x03\x02\x02\x02UX\x05\x18\r" +
		"\x02VX\x05\x1E\x10\x02WU\x03\x02\x02\x02WV\x03\x02\x02\x02X\x17\x03\x02" +
		"\x02\x02YZ\x05\x1A\x0E\x02Z[\x07\x06\x02\x02[\\\x05\x1C\x0F\x02\\\x19" +
		"\x03\x02\x02\x02]^\x07\x11\x02\x02^\x1B\x03\x02\x02\x02_`\t\x04\x02\x02" +
		"`\x1D\x03\x02\x02\x02ab\x05\x0E\b\x02bc\x07\v\x02\x02cd\x05\x0E\b\x02" +
		"de\x07\x04\x02\x02ef\x05 \x11\x02fg\x07\x05\x02\x02g\x1F\x03\x02\x02\x02" +
		"hj\x05\x18\r\x02ih\x03\x02\x02\x02jm\x03\x02\x02\x02ki\x03\x02\x02\x02" +
		"kl\x03\x02\x02\x02l!\x03\x02\x02\x02mk\x03\x02\x02\x02no\x07\f\x02\x02" +
		"op\x05\x06\x04\x02pq\x07\r\x02\x02qr\x05$\x13\x02rs\x07\x0E\x02\x02st" +
		"\x07\v\x02\x02tu\x05&\x14\x02u#\x03\x02\x02\x02v{\x05\x0E\b\x02wx\x07" +
		"\x0F\x02\x02xz\x05\x0E\b\x02yw\x03\x02\x02\x02z}\x03\x02\x02\x02{y\x03" +
		"\x02\x02\x02{|\x03\x02\x02\x02|\x7F\x03\x02\x02\x02}{\x03\x02\x02\x02" +
		"~v\x03\x02\x02\x02~\x7F\x03\x02\x02\x02\x7F%\x03\x02\x02\x02\x80\x85\x05" +
		"\x0E\b\x02\x81\x82\x07\x0F\x02\x02\x82\x84\x05\x0E\b\x02\x83\x81\x03\x02" +
		"\x02\x02\x84\x87\x03\x02\x02\x02\x85\x83\x03\x02\x02\x02\x85\x86\x03\x02" +
		"\x02\x02\x86\'\x03\x02\x02\x02\x87\x85\x03\x02\x02\x02\f+8=@RWk{~\x85";
	public static __ATN: ATN;
	public static get _ATN(): ATN {
		if (!langParser.__ATN) {
			langParser.__ATN = new ATNDeserializer().deserialize(Utils.toCharArray(langParser._serializedATN));
		}

		return langParser.__ATN;
	}

}

export class ProgramContext extends ParserRuleContext {
	public component_declaration(): Component_declarationContext[];
	public component_declaration(i: number): Component_declarationContext;
	public component_declaration(i?: number): Component_declarationContext | Component_declarationContext[] {
		if (i === undefined) {
			return this.getRuleContexts(Component_declarationContext);
		} else {
			return this.getRuleContext(i, Component_declarationContext);
		}
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_program; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterProgram) {
			listener.enterProgram(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitProgram) {
			listener.exitProgram(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitProgram) {
			return visitor.visitProgram(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Component_declarationContext extends ParserRuleContext {
	public component_name(): Component_nameContext {
		return this.getRuleContext(0, Component_nameContext);
	}
	public component_body(): Component_bodyContext {
		return this.getRuleContext(0, Component_bodyContext);
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_component_declaration; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterComponent_declaration) {
			listener.enterComponent_declaration(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitComponent_declaration) {
			listener.exitComponent_declaration(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitComponent_declaration) {
			return visitor.visitComponent_declaration(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Component_nameContext extends ParserRuleContext {
	public IDENTIFIER(): TerminalNode { return this.getToken(langParser.IDENTIFIER, 0); }
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_component_name; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterComponent_name) {
			listener.enterComponent_name(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitComponent_name) {
			listener.exitComponent_name(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitComponent_name) {
			return visitor.visitComponent_name(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Component_bodyContext extends ParserRuleContext {
	public part_or_use(): Part_or_useContext[];
	public part_or_use(i: number): Part_or_useContext;
	public part_or_use(i?: number): Part_or_useContext | Part_or_useContext[] {
		if (i === undefined) {
			return this.getRuleContexts(Part_or_useContext);
		} else {
			return this.getRuleContext(i, Part_or_useContext);
		}
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_component_body; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterComponent_body) {
			listener.enterComponent_body(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitComponent_body) {
			listener.exitComponent_body(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitComponent_body) {
			return visitor.visitComponent_body(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Part_or_useContext extends ParserRuleContext {
	public part(): PartContext | undefined {
		return this.tryGetRuleContext(0, PartContext);
	}
	public use(): UseContext | undefined {
		return this.tryGetRuleContext(0, UseContext);
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_part_or_use; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterPart_or_use) {
			listener.enterPart_or_use(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitPart_or_use) {
			listener.exitPart_or_use(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitPart_or_use) {
			return visitor.visitPart_or_use(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class PartContext extends ParserRuleContext {
	public part_name(): Part_nameContext {
		return this.getRuleContext(0, Part_nameContext);
	}
	public part_type(): Part_typeContext {
		return this.getRuleContext(0, Part_typeContext);
	}
	public part_body(): Part_bodyContext {
		return this.getRuleContext(0, Part_bodyContext);
	}
	public designator(): DesignatorContext | undefined {
		return this.tryGetRuleContext(0, DesignatorContext);
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_part; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterPart) {
			listener.enterPart(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitPart) {
			listener.exitPart(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitPart) {
			return visitor.visitPart(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Part_nameContext extends ParserRuleContext {
	public IDENTIFIER(): TerminalNode { return this.getToken(langParser.IDENTIFIER, 0); }
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_part_name; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterPart_name) {
			listener.enterPart_name(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitPart_name) {
			listener.exitPart_name(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitPart_name) {
			return visitor.visitPart_name(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class DesignatorContext extends ParserRuleContext {
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_designator; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterDesignator) {
			listener.enterDesignator(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitDesignator) {
			listener.exitDesignator(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitDesignator) {
			return visitor.visitDesignator(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Part_typeContext extends ParserRuleContext {
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_part_type; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterPart_type) {
			listener.enterPart_type(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitPart_type) {
			listener.exitPart_type(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitPart_type) {
			return visitor.visitPart_type(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Part_bodyContext extends ParserRuleContext {
	public part_body_item(): Part_body_itemContext[];
	public part_body_item(i: number): Part_body_itemContext;
	public part_body_item(i?: number): Part_body_itemContext | Part_body_itemContext[] {
		if (i === undefined) {
			return this.getRuleContexts(Part_body_itemContext);
		} else {
			return this.getRuleContext(i, Part_body_itemContext);
		}
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_part_body; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterPart_body) {
			listener.enterPart_body(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitPart_body) {
			listener.exitPart_body(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitPart_body) {
			return visitor.visitPart_body(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Part_body_itemContext extends ParserRuleContext {
	public option(): OptionContext | undefined {
		return this.tryGetRuleContext(0, OptionContext);
	}
	public connection(): ConnectionContext | undefined {
		return this.tryGetRuleContext(0, ConnectionContext);
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_part_body_item; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterPart_body_item) {
			listener.enterPart_body_item(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitPart_body_item) {
			listener.exitPart_body_item(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitPart_body_item) {
			return visitor.visitPart_body_item(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class OptionContext extends ParserRuleContext {
	public option_name(): Option_nameContext {
		return this.getRuleContext(0, Option_nameContext);
	}
	public option_value(): Option_valueContext {
		return this.getRuleContext(0, Option_valueContext);
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_option; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterOption) {
			listener.enterOption(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitOption) {
			listener.exitOption(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitOption) {
			return visitor.visitOption(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Option_nameContext extends ParserRuleContext {
	public IDENTIFIER(): TerminalNode { return this.getToken(langParser.IDENTIFIER, 0); }
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_option_name; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterOption_name) {
			listener.enterOption_name(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitOption_name) {
			listener.exitOption_name(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitOption_name) {
			return visitor.visitOption_name(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Option_valueContext extends ParserRuleContext {
	public IDENTIFIER(): TerminalNode | undefined { return this.tryGetToken(langParser.IDENTIFIER, 0); }
	public NUMBER(): TerminalNode | undefined { return this.tryGetToken(langParser.NUMBER, 0); }
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_option_value; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterOption_value) {
			listener.enterOption_value(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitOption_value) {
			listener.exitOption_value(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitOption_value) {
			return visitor.visitOption_value(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class ConnectionContext extends ParserRuleContext {
	public part_name(): Part_nameContext[];
	public part_name(i: number): Part_nameContext;
	public part_name(i?: number): Part_nameContext | Part_nameContext[] {
		if (i === undefined) {
			return this.getRuleContexts(Part_nameContext);
		} else {
			return this.getRuleContext(i, Part_nameContext);
		}
	}
	public connection_options(): Connection_optionsContext {
		return this.getRuleContext(0, Connection_optionsContext);
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_connection; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterConnection) {
			listener.enterConnection(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitConnection) {
			listener.exitConnection(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitConnection) {
			return visitor.visitConnection(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class Connection_optionsContext extends ParserRuleContext {
	public option(): OptionContext[];
	public option(i: number): OptionContext;
	public option(i?: number): OptionContext | OptionContext[] {
		if (i === undefined) {
			return this.getRuleContexts(OptionContext);
		} else {
			return this.getRuleContext(i, OptionContext);
		}
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_connection_options; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterConnection_options) {
			listener.enterConnection_options(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitConnection_options) {
			listener.exitConnection_options(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitConnection_options) {
			return visitor.visitConnection_options(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class UseContext extends ParserRuleContext {
	public component_name(): Component_nameContext {
		return this.getRuleContext(0, Component_nameContext);
	}
	public parameters(): ParametersContext {
		return this.getRuleContext(0, ParametersContext);
	}
	public outputs(): OutputsContext {
		return this.getRuleContext(0, OutputsContext);
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_use; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterUse) {
			listener.enterUse(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitUse) {
			listener.exitUse(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitUse) {
			return visitor.visitUse(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class ParametersContext extends ParserRuleContext {
	public part_name(): Part_nameContext[];
	public part_name(i: number): Part_nameContext;
	public part_name(i?: number): Part_nameContext | Part_nameContext[] {
		if (i === undefined) {
			return this.getRuleContexts(Part_nameContext);
		} else {
			return this.getRuleContext(i, Part_nameContext);
		}
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_parameters; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterParameters) {
			listener.enterParameters(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitParameters) {
			listener.exitParameters(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitParameters) {
			return visitor.visitParameters(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


export class OutputsContext extends ParserRuleContext {
	public part_name(): Part_nameContext[];
	public part_name(i: number): Part_nameContext;
	public part_name(i?: number): Part_nameContext | Part_nameContext[] {
		if (i === undefined) {
			return this.getRuleContexts(Part_nameContext);
		} else {
			return this.getRuleContext(i, Part_nameContext);
		}
	}
	constructor(parent: ParserRuleContext | undefined, invokingState: number) {
		super(parent, invokingState);
	}
	// @Override
	public get ruleIndex(): number { return langParser.RULE_outputs; }
	// @Override
	public enterRule(listener: langListener): void {
		if (listener.enterOutputs) {
			listener.enterOutputs(this);
		}
	}
	// @Override
	public exitRule(listener: langListener): void {
		if (listener.exitOutputs) {
			listener.exitOutputs(this);
		}
	}
	// @Override
	public accept<Result>(visitor: langVisitor<Result>): Result {
		if (visitor.visitOutputs) {
			return visitor.visitOutputs(this);
		} else {
			return visitor.visitChildren(this);
		}
	}
}


