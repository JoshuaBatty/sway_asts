[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe02b8c5260,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                            ),
                            start: 42,
                            end: 45,
                            as_str(): "{\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe02b8c5260,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                        ),
                        start: 33,
                        end: 45,
                        as_str(): "fn foo() {\n}",
                    },
                    return_type: Tuple(
                        [],
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe02b8c5260,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
            ),
            start: 25,
            end: 45,
            as_str(): "#[test]\nfn foo() {\n}",
        },
    },
]
