Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06d0c7010,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06d0c7010,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
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
                                            src (ptr): 0x00007fe06d0c7010,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06d0c7010,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
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
                                            src (ptr): 0x00007fe06d0c7010,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06d0c7010,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 21,
                                                    as_str(): "->",
                                                },
                                            },
                                            Str {
                                                str_token: StrToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe06d0c7010,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                                        ),
                                                        start: 22,
                                                        end: 25,
                                                        as_str(): "str",
                                                    },
                                                },
                                                length: SquareBrackets {
                                                    inner: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06d0c7010,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                                                    ),
                                                                    start: 26,
                                                                    end: 27,
                                                                    as_str(): "3",
                                                                },
                                                                parsed: 3,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe06d0c7010,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                                        ),
                                                        start: 25,
                                                        end: 28,
                                                        as_str(): "[3]",
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: Some(
                                            Literal(
                                                String(
                                                    LitString {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06d0c7010,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                                            ),
                                                            start: 35,
                                                            end: 40,
                                                            as_str(): "\"foo\"",
                                                        },
                                                        parsed: "foo",
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06d0c7010,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRce1sXG/ret_small_string/src/main.sw",
                                        ),
                                        start: 29,
                                        end: 42,
                                        as_str(): "{\n    \"foo\"\n}",
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
