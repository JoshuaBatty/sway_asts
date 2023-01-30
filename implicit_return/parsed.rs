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
                            src (ptr): 0x00007fe0ef629c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
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
                                        kind: WhileLoop(
                                            WhileLoopExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            false,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef629c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                        ),
                                                        start: 38,
                                                        end: 43,
                                                        as_str(): "false",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007fe0ef629c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                        ),
                                                        start: 44,
                                                        end: 51,
                                                        as_str(): "{\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef629c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                            ),
                                            start: 32,
                                            end: 51,
                                            as_str(): "while false {\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef629c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                    ),
                                    start: 32,
                                    end: 51,
                                    as_str(): "while false {\n    }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                42,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef629c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                            ),
                                            start: 57,
                                            end: 59,
                                            as_str(): "42",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef629c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                    ),
                                    start: 57,
                                    end: 59,
                                    as_str(): "42",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0ef629c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                            ),
                            start: 26,
                            end: 61,
                            as_str(): "{\n    while false {\n    };\n    42\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0ef629c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                        ),
                        start: 9,
                        end: 61,
                        as_str(): "fn main() -> u64 {\n    while false {\n    };\n    42\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0ef629c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                        ),
                        start: 22,
                        end: 25,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0ef629c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
            ),
            start: 9,
            end: 61,
            as_str(): "fn main() -> u64 {\n    while false {\n    };\n    42\n}",
        },
    },
]
