TyStructDeclaration {
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
    fields: [
        TyStructField {
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
            type_id: TypeId(
                2,
            ),
            initial_type_id: TypeId(
                2,
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
            attributes: {},
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
    attributes: {},
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructExpression {
                            struct_name: BaseIdent {
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
                            fields: [
                                TyStructExpressionField {
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
                                    value: TyExpression {
                                        expression: Literal(
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
                                        return_type: TypeId(
                                            8,
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
                                },
                            ],
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
                        return_type: TypeId(
                            5,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06dc66160,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRcjXi7M/ret_string_in_struct/src/main.sw",
        ),
        start: 47,
        end: 118,
        as_str(): "fn main() -> Wrapper {\n    Wrapper {\n        name: \"fuel-labs\",\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        5,
    ),
    initial_return_type: TypeId(
        4,
    ),
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

