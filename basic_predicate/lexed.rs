Some(
    LexedProgram {
        kind: Predicate,
        root: LexedModule {
            tree: Module {
                kind: Predicate {
                    predicate_token: PredicateToken {
                        span: Span {
                            src (ptr): 0x00007fb135933860,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                            ),
                            start: 0,
                            end: 9,
                            as_str(): "predicate",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb135933860,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                        ),
                        start: 9,
                        end: 10,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb135933860,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 14,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb135933860,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                            ),
                                            start: 15,
                                            end: 19,
                                            as_str(): "main",
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
                                            src (ptr): 0x00007fb135933860,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 21,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb135933860,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                                    ),
                                                    start: 22,
                                                    end: 24,
                                                    as_str(): "->",
                                                },
                                            },
                                            Path(
                                                PathType {
                                                    root_opt: None,
                                                    prefix: PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb135933860,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                                                ),
                                                                start: 25,
                                                                end: 29,
                                                                as_str(): "bool",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    suffix: [],
                                                },
                                            ),
                                        ),
                                    ),
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fb135933860,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                                            ),
                                                            start: 36,
                                                            end: 40,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb135933860,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFftyfJ/basic_predicate/src/main.sw",
                                        ),
                                        start: 30,
                                        end: 42,
                                        as_str(): "{\n    true\n}",
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
