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
                            src (ptr): 0x00007fe07c4ea020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                            ),
                            start: 80,
                            end: 84,
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
                                                    src (ptr): 0x00007fe07c4ea020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 106,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: LazyOperator(
                                                    LazyOperatorExpression {
                                                        op: And,
                                                        lhs: Expression {
                                                            kind: MethodApplication(
                                                                MethodApplicationExpression {
                                                                    method_name_binding: TypeBinding {
                                                                        inner: FromTrait {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c4ea020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                                            ),
                                                                                            start: 111,
                                                                                            end: 112,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c4ea020,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                                            ),
                                                                                            start: 111,
                                                                                            end: 112,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "lt",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c4ea020,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                                        ),
                                                                                        start: 111,
                                                                                        end: 112,
                                                                                        as_str(): "<",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c4ea020,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                            ),
                                                                            start: 111,
                                                                            end: 112,
                                                                            as_str(): "<",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Literal(
                                                                                Numeric(
                                                                                    4,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c4ea020,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "4",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Literal(
                                                                                Numeric(
                                                                                    5,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c4ea020,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                                ),
                                                                                start: 113,
                                                                                end: 114,
                                                                                as_str(): "5",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c4ea020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                ),
                                                                start: 109,
                                                                end: 114,
                                                                as_str(): "4 < 5",
                                                            },
                                                        },
                                                        rhs: Expression {
                                                            kind: Literal(
                                                                Boolean(
                                                                    false,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c4ea020,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 123,
                                                                as_str(): "false",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c4ea020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                    ),
                                                    start: 109,
                                                    end: 123,
                                                    as_str(): "4 < 5 && false",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c4ea020,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                    ),
                                    start: 101,
                                    end: 124,
                                    as_str(): "let a = 4 < 5 && false;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c4ea020,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                                    ),
                                                    start: 129,
                                                    end: 130,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c4ea020,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                            ),
                                            start: 129,
                                            end: 130,
                                            as_str(): "a",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c4ea020,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                                    ),
                                    start: 129,
                                    end: 130,
                                    as_str(): "a",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe07c4ea020,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                            ),
                            start: 95,
                            end: 132,
                            as_str(): "{\n    let a = 4 < 5 && false;\n    a\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe07c4ea020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                        ),
                        start: 77,
                        end: 132,
                        as_str(): "fn main() -> bool {\n    let a = 4 < 5 && false;\n    a\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe07c4ea020,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
                        ),
                        start: 90,
                        end: 94,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe07c4ea020,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRu3gZoK/op_precedence/src/main.sw",
            ),
            start: 77,
            end: 132,
            as_str(): "fn main() -> bool {\n    let a = 4 < 5 && false;\n    a\n}",
        },
    },
]
