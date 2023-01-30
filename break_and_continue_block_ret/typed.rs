TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: WhileLoop {
                            condition: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    6,
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
                            body: TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            6,
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
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Break,
                                                                                return_type: TypeId(
                                                                                    10,
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
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            12,
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
                                                return_type: TypeId(
                                                    15,
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
                            },
                        },
                        return_type: TypeId(
                            16,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: WhileLoop {
                            condition: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    6,
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
                            body: TyCodeBlock {
                                contents: [
                                    TyAstNode {
                                        content: ImplicitReturnExpression(
                                            TyExpression {
                                                expression: IfExp {
                                                    condition: TyExpression {
                                                        expression: Literal(
                                                            Boolean(
                                                                true,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            6,
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
                                                    then: TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Expression(
                                                                            TyExpression {
                                                                                expression: Continue,
                                                                                return_type: TypeId(
                                                                                    20,
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
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            22,
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
                                                return_type: TypeId(
                                                    25,
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
                            },
                        },
                        return_type: TypeId(
                            26,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12eb2c0e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRYJSZbq/break_and_continue_block_ret/src/main.sw",
        ),
        start: 9,
        end: 166,
        as_str(): "fn main() {\n    while true {\n        if true {\n            break;\n        }\n    }\n\n    while true {\n        if true {\n            continue;\n        }\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        4,
    ),
    initial_return_type: TypeId(
        3,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

