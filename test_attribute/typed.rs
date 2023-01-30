TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe02b8c5260,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
            ),
            start: 36,
            end: 39,
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
        src (ptr): 0x00007fe02b8c5260,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
        ),
        start: 33,
        end: 45,
        as_str(): "fn foo() {\n}",
    },
    attributes: {
        Test: [
            Attribute {
                name: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fe02b8c5260,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                        ),
                        start: 27,
                        end: 31,
                        as_str(): "test",
                    },
                    is_raw_ident: false,
                },
                args: [],
                span: Span {
                    src (ptr): 0x00007fe02b8c5260,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                    ),
                    start: 25,
                    end: 32,
                    as_str(): "#[test]",
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
        src (ptr): 0x00007fe02b8c5260,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
        ),
        start: 33,
        end: 41,
        as_str(): "fn foo()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

