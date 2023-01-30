Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb12eb2c0e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb12eb2c0e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
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
                                            src (ptr): 0x00007fb12eb2c0e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12eb2c0e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
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
                                            src (ptr): 0x00007fb12eb2c0e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12eb2c0e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                            ),
                                                            start: 25,
                                                            end: 30,
                                                            as_str(): "while",
                                                        },
                                                    },
                                                    condition: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12eb2c0e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                    ),
                                                                    start: 31,
                                                                    end: 35,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                If(
                                                                    IfExpr {
                                                                        if_token: IfToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12eb2c0e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                ),
                                                                                start: 46,
                                                                                end: 48,
                                                                                as_str(): "if",
                                                                            },
                                                                        },
                                                                        condition: Expr(
                                                                            Literal(
                                                                                Bool(
                                                                                    LitBool {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb12eb2c0e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                            ),
                                                                                            start: 49,
                                                                                            end: 53,
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
                                                                                        expr: Break {
                                                                                            break_token: BreakToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12eb2c0e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                                    ),
                                                                                                    start: 68,
                                                                                                    end: 73,
                                                                                                    as_str(): "break",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb12eb2c0e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                                    ),
                                                                                                    start: 73,
                                                                                                    end: 74,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                                final_expr_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12eb2c0e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                ),
                                                                                start: 54,
                                                                                end: 84,
                                                                                as_str(): "{\n            break;\n        }",
                                                                            },
                                                                        },
                                                                        else_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb12eb2c0e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                            ),
                                                            start: 36,
                                                            end: 90,
                                                            as_str(): "{\n        if true {\n            break;\n        }\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            While {
                                                while_token: WhileToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb12eb2c0e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                        ),
                                                        start: 96,
                                                        end: 101,
                                                        as_str(): "while",
                                                    },
                                                },
                                                condition: Literal(
                                                    Bool(
                                                        LitBool {
                                                            span: Span {
                                                                src (ptr): 0x00007fb12eb2c0e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 106,
                                                                as_str(): "true",
                                                            },
                                                            kind: True,
                                                        },
                                                    ),
                                                ),
                                                block: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            If(
                                                                IfExpr {
                                                                    if_token: IfToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12eb2c0e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                            ),
                                                                            start: 117,
                                                                            end: 119,
                                                                            as_str(): "if",
                                                                        },
                                                                    },
                                                                    condition: Expr(
                                                                        Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12eb2c0e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                        ),
                                                                                        start: 120,
                                                                                        end: 124,
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
                                                                                    expr: Continue {
                                                                                        continue_token: ContinueToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12eb2c0e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                                ),
                                                                                                start: 139,
                                                                                                end: 147,
                                                                                                as_str(): "continue",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    semicolon_token_opt: Some(
                                                                                        SemicolonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb12eb2c0e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                                ),
                                                                                                start: 147,
                                                                                                end: 148,
                                                                                                as_str(): ";",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12eb2c0e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                            ),
                                                                            start: 125,
                                                                            end: 158,
                                                                            as_str(): "{\n            continue;\n        }",
                                                                        },
                                                                    },
                                                                    else_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb12eb2c0e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                        ),
                                                        start: 107,
                                                        end: 164,
                                                        as_str(): "{\n        if true {\n            continue;\n        }\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb12eb2c0e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                        ),
                                        start: 19,
                                        end: 166,
                                        as_str(): "{\n    while true {\n        if true {\n            break;\n        }\n    }\n\n    while true {\n        if true {\n            continue;\n        }\n    }\n}",
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
