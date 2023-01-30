Some(
    LexedProgram {
        kind: Library {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe02b8c5260,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                    ),
                    start: 8,
                    end: 22,
                    as_str(): "test_attribute",
                },
                is_raw_ident: false,
            },
        },
        root: LexedModule {
            tree: Module {
                kind: Library {
                    library_token: LibraryToken {
                        span: Span {
                            src (ptr): 0x00007fe02b8c5260,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                            ),
                            start: 0,
                            end: 7,
                            as_str(): "library",
                        },
                    },
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe02b8c5260,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                            ),
                            start: 8,
                            end: 22,
                            as_str(): "test_attribute",
                        },
                        is_raw_ident: false,
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe02b8c5260,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                        ),
                        start: 22,
                        end: 23,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fe02b8c5260,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 26,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
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
                                                args: None,
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe02b8c5260,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 32,
                                        as_str(): "[test]",
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
                                            src (ptr): 0x00007fe02b8c5260,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                                            ),
                                            start: 33,
                                            end: 35,
                                            as_str(): "fn",
                                        },
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
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02b8c5260,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                                            ),
                                            start: 39,
                                            end: 41,
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
                                        src (ptr): 0x00007fe02b8c5260,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRTf0CWG/test_attribute/src/main.sw",
                                        ),
                                        start: 42,
                                        end: 45,
                                        as_str(): "{\n}",
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
