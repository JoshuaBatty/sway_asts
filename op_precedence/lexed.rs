Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe07c4ea020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe07c4ea020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe07c4ea020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 79,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c4ea020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                            ),
                                            start: 80,
                                            end: 84,
                                            as_str(): "main",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c4ea020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                            ),
                                            start: 84,
                                            end: 86,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe07c4ea020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 89,
                                                    as_str(): "->",
                                                },
                                            },
                                            Path(
                                                PathType {
                                                    root_opt: None,
                                                    prefix: PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c4ea020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 94,
                                                                as_str(): "bool",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    suffix: [],
                                                },
                                            ),
                                        ),
                                    ),
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c4ea020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 104,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c4ea020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 106,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c4ea020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                            ),
                                                            start: 107,
                                                            end: 108,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: LogicalAnd {
                                                        lhs: LessThan {
                                                            lhs: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c4ea020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                            ),
                                                                            start: 109,
                                                                            end: 110,
                                                                            as_str(): "4",
                                                                        },
                                                                        parsed: 4,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            less_than_token: LessThanToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c4ea020,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 112,
                                                                    as_str(): "<",
                                                                },
                                                            },
                                                            rhs: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c4ea020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 114,
                                                                            as_str(): "5",
                                                                        },
                                                                        parsed: 5,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        double_ampersand_token: DoubleAmpersandToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c4ea020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                ),
                                                                start: 115,
                                                                end: 117,
                                                                as_str(): "&&",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Bool(
                                                                LitBool {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c4ea020,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                        ),
                                                                        start: 118,
                                                                        end: 123,
                                                                        as_str(): "false",
                                                                    },
                                                                    kind: False,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c4ea020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                            ),
                                                            start: 123,
                                                            end: 124,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c4ea020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 130,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    suffix: [],
                                                    incomplete_suffix: false,
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe07c4ea020,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 132,
                                        as_str(): "{\n    let a = 4 < 5 && false;\n    a\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [],
        },
    },
)
