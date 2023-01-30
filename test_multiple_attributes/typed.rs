TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe05cc7bb00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
            ),
            start: 62,
            end: 65,
            as_str(): "foo",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05cc7bb00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
        ),
        start: 59,
        end: 70,
        as_str(): "fn foo() {}",
    },
    attributes: {
        Test: [
            Attribute {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fe05cc7bb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                        ),
                        start: 37,
                        end: 41,
                        as_str(): "test",
                    },
                    is_raw_ident: false,
                },
                args: [],
                span: Span {
                    src (ptr): 0x00007fe05cc7bb00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                    ),
                    start: 35,
                    end: 58,
                    as_str(): "#[test, inline(always)]",
                },
            },
        ],
        Inline: [
            Attribute {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fe05cc7bb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                        ),
                        start: 43,
                        end: 49,
                        as_str(): "inline",
                    },
                    is_raw_ident: false,
                },
                args: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe05cc7bb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                            ),
                            start: 50,
                            end: 56,
                            as_str(): "always",
                        },
                        is_raw_ident: false,
                    },
                ],
                span: Span {
                    src (ptr): 0x00007fe05cc7bb00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                    ),
                    start: 35,
                    end: 58,
                    as_str(): "#[test, inline(always)]",
                },
            },
        ],
    },
    return_type: TypeId(
        4,
    ),
    initial_return_type: TypeId(
        3,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe05cc7bb00,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
        ),
        start: 59,
        end: 67,
        as_str(): "fn foo()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

