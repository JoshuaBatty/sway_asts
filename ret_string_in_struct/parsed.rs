[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06dc66160,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                            ),
                            start: 16,
                            end: 23,
                            as_str(): "Wrapper",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06dc66160,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                    ),
                                    start: 30,
                                    end: 34,
                                    as_str(): "name",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Str(
                                Length {
                                    val: 9,
                                    span: Span {
                                        src (ptr): 0x00007fe06dc66160,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                        ),
                                        start: 40,
                                        end: 41,
                                        as_str(): "9",
                                    },
                                },
                            ),
                            span: Span {
                                src (ptr): 0x00007fe06dc66160,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                ),
                                start: 30,
                                end: 42,
                                as_str(): "name: str[9]",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe06dc66160,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                ),
                                start: 36,
                                end: 42,
                                as_str(): "str[9]",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe06dc66160,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                        ),
                        start: 9,
                        end: 45,
                        as_str(): "struct Wrapper {\n    name: str[9],\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06dc66160,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
            ),
            start: 9,
            end: 45,
            as_str(): "struct Wrapper {\n    name: str[9],\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06dc66160,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                            ),
                            start: 50,
                            end: 54,
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
                                        kind: Struct(
                                            StructExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06dc66160,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 81,
                                                                as_str(): "Wrapper",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe06dc66160,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                        ),
                                                        start: 74,
                                                        end: 81,
                                                        as_str(): "Wrapper",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06dc66160,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 96,
                                                                as_str(): "name",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                String(
                                                                    Span {
                                                                        src (ptr): 0x00007fe06dc66160,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 99,
                                                                        end: 108,
                                                                        as_str(): "fuel-labs",
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe06dc66160,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 109,
                                                                as_str(): "\"fuel-labs\"",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06dc66160,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 109,
                                                            as_str(): "name: \"fuel-labs\"",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06dc66160,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                            ),
                                            start: 74,
                                            end: 116,
                                            as_str(): "Wrapper {\n        name: \"fuel-labs\",\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06dc66160,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                    ),
                                    start: 74,
                                    end: 116,
                                    as_str(): "Wrapper {\n        name: \"fuel-labs\",\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06dc66160,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                            ),
                            start: 68,
                            end: 118,
                            as_str(): "{\n    Wrapper {\n        name: \"fuel-labs\",\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06dc66160,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                        ),
                        start: 47,
                        end: 118,
                        as_str(): "fn main() -> Wrapper {\n    Wrapper {\n        name: \"fuel-labs\",\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe06dc66160,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                                ),
                                start: 60,
                                end: 67,
                                as_str(): "Wrapper",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06dc66160,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
                        ),
                        start: 60,
                        end: 67,
                        as_str(): "Wrapper",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06dc66160,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
            ),
            start: 47,
            end: 118,
            as_str(): "fn main() -> Wrapper {\n    Wrapper {\n        name: \"fuel-labs\",\n    }\n}",
        },
    },
]
