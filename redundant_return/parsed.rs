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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05f893370,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                        ),
                                                        start: 35,
                                                        end: 39,
                                                        as_str(): "true",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Return(
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            1,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05f893370,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                        ),
                                                                                        start: 57,
                                                                                        end: 58,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05f893370,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                ),
                                                                                start: 50,
                                                                                end: 58,
                                                                                as_str(): "return 1",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05f893370,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                        ),
                                                                        start: 50,
                                                                        end: 58,
                                                                        as_str(): "return 1",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe05f893370,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                ),
                                                                start: 40,
                                                                end: 65,
                                                                as_str(): "{\n        return 1;\n    }",
                                                            },
                                                        },
                                                    ),
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
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: Expression(
                                                                            Expression {
                                                                                kind: Return(
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                0,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05f893370,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                            ),
                                                                                            start: 88,
                                                                                            end: 89,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05f893370,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 89,
                                                                                    as_str(): "return 0",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05f893370,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                                            ),
                                                                            start: 81,
                                                                            end: 89,
                                                                            as_str(): "return 0",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
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
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05f893370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                            ),
                                            start: 32,
                                            end: 96,
                                            as_str(): "if true {\n        return 1;\n    } else {\n        return 0;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05f893370,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 96,
                                    as_str(): "if true {\n        return 1;\n    } else {\n        return 0;\n    }",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        2,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05f893370,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                                    ),
                                                    start: 108,
                                                    end: 109,
                                                    as_str(): "2",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05f893370,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                            ),
                                            start: 101,
                                            end: 109,
                                            as_str(): "return 2",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05f893370,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                                    ),
                                    start: 101,
                                    end: 109,
                                    as_str(): "return 2",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05f893370,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                            ),
                            start: 26,
                            end: 112,
                            as_str(): "{\n    if true {\n        return 1;\n    } else {\n        return 0;\n    }\n    return 2;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe05f893370,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                        ),
                        start: 9,
                        end: 112,
                        as_str(): "fn main() -> u64 {\n    if true {\n        return 1;\n    } else {\n        return 0;\n    }\n    return 2;\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05f893370,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05f893370,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
            ),
            start: 9,
            end: 112,
            as_str(): "fn main() -> u64 {\n    if true {\n        return 1;\n    } else {\n        return 0;\n    }\n    return 2;\n}",
        },
    },
]
