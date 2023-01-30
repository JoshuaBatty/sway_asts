TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb125da2240,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
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
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb125da2240,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 40,
                                    as_str(): "addr",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12b3ec490,
                                            path: None,
                                            start: 6,
                                            end: 24,
                                            as_str(): "some_contract_addr",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb125da2240,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 61,
                                        as_str(): "some_contract_addr",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    2,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb125da2240,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                    ),
                                    start: 43,
                                    end: 61,
                                    as_str(): "some_contract_addr",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                2,
                            ),
                            type_ascription: TypeId(
                                13,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb125da2240,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                    ),
                    start: 32,
                    end: 62,
                    as_str(): "let addr = some_contract_addr;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: IfExp {
                                    condition: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12e8141b0,
                                                    path: None,
                                                    start: 6,
                                                    end: 15,
                                                    as_str(): "true_bool",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb125da2240,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                ),
                                                start: 78,
                                                end: 87,
                                                as_str(): "true_bool",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            9,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb125da2240,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 87,
                                            as_str(): "true_bool",
                                        },
                                    },
                                    then: TyExpression {
                                        expression: CodeBlock(
                                            TyCodeBlock {
                                                contents: [
                                                    TyAstNode {
                                                        content: ImplicitReturnExpression(
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb126d0aa80,
                                                                            path: None,
                                                                            start: 6,
                                                                            end: 14,
                                                                            as_str(): "some_num",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb125da2240,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                        ),
                                                                        start: 90,
                                                                        end: 98,
                                                                        as_str(): "some_num",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    5,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb125da2240,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                    ),
                                                                    start: 90,
                                                                    end: 98,
                                                                    as_str(): "some_num",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb125da2240,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 98,
                                                            as_str(): "some_num",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        return_type: TypeId(
                                            5,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb125da2240,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                            ),
                                            start: 88,
                                            end: 100,
                                            as_str(): "{ some_num }",
                                        },
                                    },
                                    else: Some(
                                        TyExpression {
                                            expression: CodeBlock(
                                                TyCodeBlock {
                                                    contents: [
                                                        TyAstNode {
                                                            content: ImplicitReturnExpression(
                                                                TyExpression {
                                                                    expression: Literal(
                                                                        U64(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        5,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb125da2240,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                        ),
                                                                        start: 108,
                                                                        end: 109,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb125da2240,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                ),
                                                                start: 108,
                                                                end: 109,
                                                                as_str(): "0",
                                                            },
                                                        },
                                                    ],
                                                },
                                            ),
                                            return_type: TypeId(
                                                5,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb125da2240,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                ),
                                                start: 106,
                                                end: 111,
                                                as_str(): "{ 0 }",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb125da2240,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                    ),
                                    start: 75,
                                    end: 111,
                                    as_str(): "if true_bool { some_num } else { 0 }",
                                },
                            },
                        ),
                        return_type: TypeId(
                            19,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb125da2240,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                            ),
                            start: 68,
                            end: 111,
                            as_str(): "return if true_bool { some_num } else { 0 }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb125da2240,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                    ),
                    start: 68,
                    end: 111,
                    as_str(): "return if true_bool { some_num } else { 0 }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb125da2240,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
        ),
        start: 9,
        end: 115,
        as_str(): "fn main() -> u64 {\n    let addr = some_contract_addr;\n\n    return if true_bool { some_num } else { 0 } ;\n}",
    },
    attributes: {},
    return_type: TypeId(
        5,
    ),
    initial_return_type: TypeId(
        5,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb125da2240,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
        ),
        start: 22,
        end: 25,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

