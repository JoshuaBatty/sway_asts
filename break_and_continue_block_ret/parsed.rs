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
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb12eb2c0e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                        ),
                                                        start: 31,
                                                        end: 35,
                                                        as_str(): "true",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: ImplicitReturnExpression(
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
                                                                                    src (ptr): 0x00007fb12eb2c0e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                    ),
                                                                                    start: 49,
                                                                                    end: 53,
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
                                                                                                        kind: Break,
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
                                                                                                ),
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
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fb12eb2c0e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                            ),
                                                                                            start: 54,
                                                                                            end: 84,
                                                                                            as_str(): "{\n            break;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
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
                                                                            else: None,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12eb2c0e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                        ),
                                                                        start: 46,
                                                                        end: 84,
                                                                        as_str(): "if true {\n            break;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12eb2c0e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                ),
                                                                start: 46,
                                                                end: 84,
                                                                as_str(): "if true {\n            break;\n        }",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
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
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12eb2c0e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                            ),
                                            start: 25,
                                            end: 90,
                                            as_str(): "while true {\n        if true {\n            break;\n        }\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12eb2c0e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                    ),
                                    start: 25,
                                    end: 90,
                                    as_str(): "while true {\n        if true {\n            break;\n        }\n    }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
                                                condition: Expression {
                                                    kind: Literal(
                                                        Boolean(
                                                            true,
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb12eb2c0e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                        ),
                                                        start: 102,
                                                        end: 106,
                                                        as_str(): "true",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: ImplicitReturnExpression(
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
                                                                                    src (ptr): 0x00007fb12eb2c0e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                    ),
                                                                                    start: 120,
                                                                                    end: 124,
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
                                                                                                        kind: Continue,
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
                                                                                                ),
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
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fb12eb2c0e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                                            ),
                                                                                            start: 125,
                                                                                            end: 158,
                                                                                            as_str(): "{\n            continue;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
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
                                                                            else: None,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb12eb2c0e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 158,
                                                                        as_str(): "if true {\n            continue;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb12eb2c0e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                                                ),
                                                                start: 117,
                                                                end: 158,
                                                                as_str(): "if true {\n            continue;\n        }",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
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
                                        span: Span {
                                            src (ptr): 0x00007fb12eb2c0e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                            ),
                                            start: 96,
                                            end: 164,
                                            as_str(): "while true {\n        if true {\n            continue;\n        }\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12eb2c0e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                                    ),
                                    start: 96,
                                    end: 164,
                                    as_str(): "while true {\n        if true {\n            continue;\n        }\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12eb2c0e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                            ),
                            start: 19,
                            end: 166,
                            as_str(): "{\n    while true {\n        if true {\n            break;\n        }\n    }\n\n    while true {\n        if true {\n            continue;\n        }\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12eb2c0e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                        ),
                        start: 9,
                        end: 166,
                        as_str(): "fn main() {\n    while true {\n        if true {\n            break;\n        }\n    }\n\n    while true {\n        if true {\n            continue;\n        }\n    }\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12eb2c0e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
                        ),
                        start: 9,
                        end: 18,
                        as_str(): "fn main()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12eb2c0e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
            ),
            start: 9,
            end: 166,
            as_str(): "fn main() {\n    while true {\n        if true {\n            break;\n        }\n    }\n\n    while true {\n        if true {\n            continue;\n        }\n    }\n}",
        },
    },
]
