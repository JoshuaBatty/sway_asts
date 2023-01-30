[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0bd033020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                            ),
                            start: 120,
                            end: 124,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_1",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 147,
                                                                            end: 151,
                                                                            as_str(): "true",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Literal(
                                                                            Boolean(
                                                                                true,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bd033020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                            ),
                                                                            start: 147,
                                                                            end: 151,
                                                                            as_str(): "true",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd033020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                            ),
                                                            start: 141,
                                                            end: 268,
                                                            as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_1",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 147,
                                                                                        end: 151,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                ),
                                                                                start: 147,
                                                                                end: 151,
                                                                                as_str(): "true",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: Boolean(
                                                                                        true,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 162,
                                                                                        end: 166,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [
                                                                                                AstNode {
                                                                                                    content: Expression(
                                                                                                        Expression {
                                                                                                            kind: Return(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Boolean(
                                                                                                                            true,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 191,
                                                                                                                        end: 195,
                                                                                                                        as_str(): "true",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                                ),
                                                                                                                start: 184,
                                                                                                                end: 195,
                                                                                                                as_str(): "return true",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                        ),
                                                                                                        start: 184,
                                                                                                        end: 195,
                                                                                                        as_str(): "return true",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                ),
                                                                                                start: 170,
                                                                                                end: 206,
                                                                                                as_str(): "{\n            return true;\n        }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 170,
                                                                                        end: 206,
                                                                                        as_str(): "{\n            return true;\n        }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd033020,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                    ),
                                                                                    start: 162,
                                                                                    end: 207,
                                                                                    as_str(): "true => {\n            return true;\n        },",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: Literal {
                                                                                    value: Boolean(
                                                                                        false,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 216,
                                                                                        end: 221,
                                                                                        as_str(): "false",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [
                                                                                                AstNode {
                                                                                                    content: Expression(
                                                                                                        Expression {
                                                                                                            kind: Return(
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Boolean(
                                                                                                                            false,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 246,
                                                                                                                        end: 251,
                                                                                                                        as_str(): "false",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                                ),
                                                                                                                start: 239,
                                                                                                                end: 251,
                                                                                                                as_str(): "return false",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                        ),
                                                                                                        start: 239,
                                                                                                        end: 251,
                                                                                                        as_str(): "return false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe0bd033020,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                                ),
                                                                                                start: 225,
                                                                                                end: 262,
                                                                                                as_str(): "{\n            return false;\n        }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bd033020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                        ),
                                                                                        start: 225,
                                                                                        end: 262,
                                                                                        as_str(): "{\n            return false;\n        }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0bd033020,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                                    ),
                                                                                    start: 216,
                                                                                    end: 262,
                                                                                    as_str(): "false => {\n            return false;\n        }",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bd033020,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                                    ),
                                                                    start: 141,
                                                                    end: 268,
                                                                    as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bd033020,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                            ),
                                                            start: 141,
                                                            end: 268,
                                                            as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe0bd033020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                                    ),
                                                    start: 141,
                                                    end: 268,
                                                    as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0bd033020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                            ),
                                            start: 141,
                                            end: 268,
                                            as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0bd033020,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                                    ),
                                    start: 141,
                                    end: 268,
                                    as_str(): "match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0bd033020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                            ),
                            start: 135,
                            end: 270,
                            as_str(): "{\n    match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0bd033020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                        ),
                        start: 117,
                        end: 270,
                        as_str(): "fn main() -> bool {\n    match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0bd033020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
                        ),
                        start: 130,
                        end: 134,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0bd033020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRpDulHL/match_expressions_explicit_rets/src/main.sw",
            ),
            start: 117,
            end: 270,
            as_str(): "fn main() -> bool {\n    match true {\n        true => {\n            return true;\n        },\n        false => {\n            return false;\n        }\n    }\n}",
        },
    },
]
