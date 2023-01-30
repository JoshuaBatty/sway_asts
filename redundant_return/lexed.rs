Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe05f893370,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe05f893370,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
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
                                            src (ptr): 0x00007fe05f893370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05f893370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
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
                                            src (ptr): 0x00007fe05f893370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05f893370,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 21,
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
                                                                src (ptr): 0x00007fe05f893370,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 25,
                                                                as_str(): "u64",
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
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe05f893370,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                ),
                                                                start: 32,
                                                                end: 34,
                                                                as_str(): "if",
                                                            },
                                                        },
                                                        condition: Expr(
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05f893370,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                            ),
                                                                            start: 35,
                                                                            end: 39,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        then_block: Braces {
                                                            inner: CodeBlockContents {
                                                                statements: [
                                                                    Expr {
                                                                        expr: Return {
                                                                            return_token: ReturnToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05f893370,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                    ),
                                                                                    start: 50,
                                                                                    end: 56,
                                                                                    as_str(): "return",
                                                                                },
                                                                            },
                                                                            expr_opt: Some(
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05f893370,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                                ),
                                                                                                start: 57,
                                                                                                end: 58,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05f893370,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                    ),
                                                                                    start: 58,
                                                                                    end: 59,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe05f893370,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                ),
                                                                start: 40,
                                                                end: 65,
                                                                as_str(): "{\n        return 1;\n    }",
                                                            },
                                                        },
                                                        else_opt: Some(
                                                            (
                                                                ElseToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05f893370,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                        ),
                                                                        start: 66,
                                                                        end: 70,
                                                                        as_str(): "else",
                                                                    },
                                                                },
                                                                Break(
                                                                    Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [
                                                                                Expr {
                                                                                    expr: Return {
                                                                                        return_token: ReturnToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05f893370,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                                ),
                                                                                                start: 81,
                                                                                                end: 87,
                                                                                                as_str(): "return",
                                                                                            },
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe05f893370,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                                            ),
                                                                                                            start: 88,
                                                                                                            end: 89,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                        parsed: 0,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    semicolon_token_opt: Some(
                                                                                        SemicolonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05f893370,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                                ),
                                                                                                start: 89,
                                                                                                end: 90,
                                                                                                as_str(): ";",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05f893370,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                            ),
                                                                            start: 71,
                                                                            end: 96,
                                                                            as_str(): "{\n        return 0;\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                semicolon_token_opt: None,
                                            },
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05f893370,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 107,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05f893370,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                        ),
                                                                        start: 108,
                                                                        end: 109,
                                                                        as_str(): "2",
                                                                    },
                                                                    parsed: 2,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05f893370,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                            ),
                                                            start: 109,
                                                            end: 110,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05f893370,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 112,
                                        as_str(): "{\n    if true {\n        return 1;\n    } else {\n        return 0;\n    }\n    return 2;\n}",
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
