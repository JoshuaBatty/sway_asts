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
                    visibility: Private,
                    body: CodeBlock {
                        contents: [],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05cc7bb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                            ),
                            start: 68,
                            end: 70,
                            as_str(): "{}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe05cc7bb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                        ),
                        start: 59,
                        end: 70,
                        as_str(): "fn foo() {}",
                    },
                    return_type: Tuple(
                        [],
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05cc7bb00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
            ),
            start: 35,
            end: 70,
            as_str(): "#[test, inline(always)]\nfn foo() {}",
        },
    },
]
