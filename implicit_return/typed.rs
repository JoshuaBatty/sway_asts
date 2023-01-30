TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: WhileLoop {
                            condition: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        false,
                                    ),
                                ),
                                return_type: TypeId(
                                    5,
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
                            body: TyCodeBlock {
                                contents: [],
                            },
                        },
                        return_type: TypeId(
                            8,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                42,
                            ),
                        ),
                        return_type: TypeId(
                            9,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0ef629c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
        ),
        start: 9,
        end: 61,
        as_str(): "fn main() -> u64 {\n    while false {\n    };\n    42\n}",
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
        src (ptr): 0x00007fe0ef629c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

