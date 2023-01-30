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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
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
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Variable(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb125da2240,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                            ),
                                                            start: 43,
                                                            end: 61,
                                                            as_str(): "some_contract_addr",
                                                        },
                                                        is_raw_ident: false,
                                                    },
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
                                            is_mutable: false,
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
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: If(
                                                    IfExpression {
                                                        condition: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb125da2240,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                        ),
                                                                        start: 78,
                                                                        end: 87,
                                                                        as_str(): "true_bool",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
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
                                                        then: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb125da2240,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 90,
                                                                                                end: 98,
                                                                                                as_str(): "some_num",
                                                                                            },
                                                                                            is_raw_ident: false,
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
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fb125da2240,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                        ),
                                                                        start: 88,
                                                                        end: 100,
                                                                        as_str(): "{ some_num }",
                                                                    },
                                                                },
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
                                                            Expression {
                                                                kind: CodeBlock(
                                                                    CodeBlock {
                                                                        contents: [
                                                                            AstNode {
                                                                                content: ImplicitReturnExpression(
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                0,
                                                                                            ),
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
                                                                        whole_block_span: Span {
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
                        whole_block_span: Span {
                            src (ptr): 0x00007fb125da2240,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                            ),
                            start: 26,
                            end: 115,
                            as_str(): "{\n    let addr = some_contract_addr;\n\n    return if true_bool { some_num } else { 0 } ;\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb125da2240,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                        ),
                        start: 9,
                        end: 115,
                        as_str(): "fn main() -> u64 {\n    let addr = some_contract_addr;\n\n    return if true_bool { some_num } else { 0 } ;\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb125da2240,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
            ),
            start: 9,
            end: 115,
            as_str(): "fn main() -> u64 {\n    let addr = some_contract_addr;\n\n    return if true_bool { some_num } else { 0 } ;\n}",
        },
    },
]
