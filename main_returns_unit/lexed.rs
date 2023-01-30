Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0c8c39d20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0c8c39d20,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
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
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 15,
                                            as_str(): "ten",
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
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 15,
                                            end: 17,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0c8c39d20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                    ),
                                                    start: 18,
                                                    end: 20,
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
                                                                src (ptr): 0x00007fe0c8c39d20,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                                ),
                                                                start: 21,
                                                                end: 24,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8c39d20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                            ),
                                                            start: 31,
                                                            end: 33,
                                                            as_str(): "10",
                                                        },
                                                        parsed: 10,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c8c39d20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 35,
                                        as_str(): "{\n    10\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 37,
                                            end: 39,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 43,
                                            as_str(): "nop",
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
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 43,
                                            end: 45,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8c39d20,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                                        ),
                                                                        start: 52,
                                                                        end: 55,
                                                                        as_str(): "ten",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8c39d20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                            ),
                                                            start: 55,
                                                            end: 57,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8c39d20,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 58,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c8c39d20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                        ),
                                        start: 46,
                                        end: 60,
                                        as_str(): "{\n    ten();\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 62,
                                            end: 64,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 65,
                                            end: 69,
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
                                            src (ptr): 0x00007fe0c8c39d20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                            ),
                                            start: 69,
                                            end: 71,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: Some(
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8c39d20,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                                    ),
                                                                    start: 78,
                                                                    end: 81,
                                                                    as_str(): "nop",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                        incomplete_suffix: false,
                                                    },
                                                ),
                                                args: Parens {
                                                    inner: Punctuated {
                                                        value_separator_pairs: [],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c8c39d20,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                                        ),
                                                        start: 81,
                                                        end: 83,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c8c39d20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgKD0P/main_returns_unit/src/main.sw",
                                        ),
                                        start: 72,
                                        end: 85,
                                        as_str(): "{\n    nop()\n}",
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
