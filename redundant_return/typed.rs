TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
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
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Return(
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        1,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    3,
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
                                                        return_type: TypeId(
                                                            10,
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
                                    },
                                ),
                                return_type: TypeId(
                                    11,
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
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: Expression(
                                                        TyExpression {
                                                            expression: Return(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        3,
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
                                                            return_type: TypeId(
                                                                16,
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
                                        },
                                    ),
                                    return_type: TypeId(
                                        17,
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
                        return_type: TypeId(
                            17,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: Literal(
                                    U64(
                                        2,
                                    ),
                                ),
                                return_type: TypeId(
                                    3,
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
                        return_type: TypeId(
                            21,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05f893370,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRKr3Y2c/redundant_return/src/main.sw",
        ),
        start: 9,
        end: 112,
        as_str(): "fn main() -> u64 {\n    if true {\n        return 1;\n    } else {\n        return 0;\n    }\n    return 2;\n}",
    },
    attributes: {},
    return_type: TypeId(
        3,
    ),
    initial_return_type: TypeId(
        3,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

