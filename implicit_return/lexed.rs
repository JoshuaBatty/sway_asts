Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0ef629c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0ef629c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
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
                                            src (ptr): 0x00007fe0ef629c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ef629c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
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
                                            src (ptr): 0x00007fe0ef629c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
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
                                                    src (ptr): 0x00007fe0ef629c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 21,
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
                                                                src (ptr): 0x00007fe0ef629c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 25,
                                                                as_str(): "u64",
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
                                        statements: [
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef629c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                            ),
                                                            start: 32,
                                                            end: 37,
                                                            as_str(): "while",
                                                        },
                                                    },
                                                    condition: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef629c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                                    ),
                                                                    start: 38,
                                                                    end: 43,
                                                                    as_str(): "false",
                                                                },
                                                                kind: False,
                                                            },
                                                        ),
                                                    ),
                                                    block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef629c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                            ),
                                                            start: 44,
                                                            end: 51,
                                                            as_str(): "{\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef629c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                            ),
                                                            start: 51,
                                                            end: 52,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef629c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 59,
                                                            as_str(): "42",
                                                        },
                                                        parsed: 42,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0ef629c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT5V1pA/implicit_return/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 61,
                                        as_str(): "{\n    while false {\n    };\n    42\n}",
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
