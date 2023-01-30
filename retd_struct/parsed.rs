[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe05bcc9310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                            ),
                            start: 16,
                            end: 31,
                            as_str(): "BiggerThanAWord",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05bcc9310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                    ),
                                    start: 38,
                                    end: 45,
                                    as_str(): "field_1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe05bcc9310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                ),
                                start: 38,
                                end: 50,
                                as_str(): "field_1: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe05bcc9310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                ),
                                start: 47,
                                end: 50,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05bcc9310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                    ),
                                    start: 56,
                                    end: 63,
                                    as_str(): "field_2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: B256,
                            span: Span {
                                src (ptr): 0x00007fe05bcc9310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                ),
                                start: 56,
                                end: 69,
                                as_str(): "field_2: b256",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe05bcc9310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                ),
                                start: 65,
                                end: 69,
                                as_str(): "b256",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe05bcc9310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                        ),
                        start: 9,
                        end: 72,
                        as_str(): "struct BiggerThanAWord {\n    field_1: u64,\n    field_2: b256,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05bcc9310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
            ),
            start: 9,
            end: 72,
            as_str(): "struct BiggerThanAWord {\n    field_1: u64,\n    field_2: b256,\n}",
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
                            src (ptr): 0x00007fe05bcc9310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                            ),
                            start: 77,
                            end: 81,
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
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 109,
                                                                end: 124,
                                                                as_str(): "BiggerThanAWord",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe05bcc9310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                        ),
                                                        start: 109,
                                                        end: 124,
                                                        as_str(): "BiggerThanAWord",
                                                    },
                                                },
                                                fields: [
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 135,
                                                                end: 142,
                                                                as_str(): "field_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                U64(
                                                                    99999,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 144,
                                                                end: 152,
                                                                as_str(): "99999u64",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05bcc9310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 152,
                                                            as_str(): "field_1: 99999u64",
                                                        },
                                                    },
                                                    StructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 162,
                                                                end: 169,
                                                                as_str(): "field_2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: Expression {
                                                            kind: Literal(
                                                                B256(
                                                                    [
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                        255,
                                                                    ],
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe05bcc9310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                                ),
                                                                start: 171,
                                                                end: 237,
                                                                as_str(): "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05bcc9310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 237,
                                                            as_str(): "field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05bcc9310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                            ),
                                            start: 109,
                                            end: 244,
                                            as_str(): "BiggerThanAWord {\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05bcc9310,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                    ),
                                    start: 109,
                                    end: 244,
                                    as_str(): "BiggerThanAWord {\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05bcc9310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                            ),
                            start: 103,
                            end: 246,
                            as_str(): "{\n    BiggerThanAWord {\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe05bcc9310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                        ),
                        start: 74,
                        end: 246,
                        as_str(): "fn main() -> BiggerThanAWord {\n    BiggerThanAWord {\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe05bcc9310,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                                ),
                                start: 87,
                                end: 102,
                                as_str(): "BiggerThanAWord",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05bcc9310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
                        ),
                        start: 87,
                        end: 102,
                        as_str(): "BiggerThanAWord",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05bcc9310,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
            ),
            start: 74,
            end: 246,
            as_str(): "fn main() -> BiggerThanAWord {\n    BiggerThanAWord {\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }\n}",
        },
    },
]
