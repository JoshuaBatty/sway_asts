Some(
    LexedProgram {
        kind: Library {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe05cc7bb00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                    ),
                    start: 8,
                    end: 32,
                    as_str(): "test_multiple_attributes",
                },
                is_raw_ident: false,
            },
        },
        root: LexedModule {
            tree: Module {
                kind: Library {
                    library_token: LibraryToken {
                        span: Span {
                            src (ptr): 0x00007fe05cc7bb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                            ),
                            start: 0,
                            end: 7,
                            as_str(): "library",
                        },
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe05cc7bb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                            ),
                            start: 8,
                            end: 32,
                            as_str(): "test_multiple_attributes",
                        },
                        is_raw_ident: false,
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe05cc7bb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                        ),
                        start: 32,
                        end: 33,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fe05cc7bb00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 36,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
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
                                                    args: None,
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe05cc7bb00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                                                        ),
                                                        start: 41,
                                                        end: 42,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: Some(
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
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05cc7bb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                                                            ),
                                                            start: 49,
                                                            end: 57,
                                                            as_str(): "(always)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05cc7bb00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                                        ),
                                        start: 36,
                                        end: 58,
                                        as_str(): "[test, inline(always)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe05cc7bb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 61,
                                            as_str(): "fn",
                                        },
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
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05cc7bb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 67,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05cc7bb00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqyUxVI/test_multiple_attributes/src/main.sw",
                                        ),
                                        start: 68,
                                        end: 70,
                                        as_str(): "{}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [],
        },
    },
)
