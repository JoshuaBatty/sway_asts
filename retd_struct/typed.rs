TyStructDeclaration {
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
    fields: [
        TyStructField {
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
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
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
            attributes: {},
        },
        TyStructField {
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
            type_id: TypeId(
                59,
            ),
            initial_type_id: TypeId(
                59,
            ),
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
            attributes: {},
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
    attributes: {},
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructExpression {
                            struct_name: BaseIdent {
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
                            fields: [
                                TyStructExpressionField {
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
                                    value: TyExpression {
                                        expression: Literal(
                                            U64(
                                                99999,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
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
                                },
                                TyStructExpressionField {
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
                                    value: TyExpression {
                                        expression: Literal(
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
                                        return_type: TypeId(
                                            59,
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
                                },
                            ],
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
                        return_type: TypeId(
                            7253,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05bcc9310,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIReFaD8c/retd_struct/src/main.sw",
        ),
        start: 74,
        end: 246,
        as_str(): "fn main() -> BiggerThanAWord {\n    BiggerThanAWord {\n        field_1: 99999u64,\n        field_2: 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

