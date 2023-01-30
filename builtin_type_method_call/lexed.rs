Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb126e2ab50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb126e2ab50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fb126e2ab50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb126e2ab50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 17,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb126e2ab50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                            ),
                                            start: 17,
                                            end: 19,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb126e2ab50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 22,
                                                as_str(): "ops",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb126e2ab50,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                ),
                                                start: 22,
                                                end: 24,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Glob {
                                            star_token: StarToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb126e2ab50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
                                                    as_str(): "*",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb126e2ab50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 26,
                                        as_str(): ";",
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
                                            src (ptr): 0x00007fb126e2ab50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                            ),
                                            start: 28,
                                            end: 30,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb126e2ab50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                            ),
                                            start: 31,
                                            end: 35,
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
                                            src (ptr): 0x00007fb126e2ab50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb126e2ab50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                    ),
                                                    start: 38,
                                                    end: 40,
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
                                                                src (ptr): 0x00007fb126e2ab50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                ),
                                                                start: 41,
                                                                end: 44,
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
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb126e2ab50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                            ),
                                                            start: 51,
                                                            end: 54,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb126e2ab50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 56,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb126e2ab50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 58,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb126e2ab50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                    ),
                                                                    start: 59,
                                                                    end: 60,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: Some(
                                                                    (
                                                                        U64,
                                                                        Span {
                                                                            src (ptr): 0x00007fb126e2ab50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                            ),
                                                                            start: 60,
                                                                            end: 63,
                                                                            as_str(): "u64",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb126e2ab50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                            ),
                                                            start: 63,
                                                            end: 64,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb126e2ab50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                            ),
                                                            start: 69,
                                                            end: 72,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb126e2ab50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 74,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb126e2ab50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 76,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb126e2ab50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                    ),
                                                                    start: 77,
                                                                    end: 78,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: Some(
                                                                    (
                                                                        U64,
                                                                        Span {
                                                                            src (ptr): 0x00007fb126e2ab50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 81,
                                                                            as_str(): "u64",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb126e2ab50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                            ),
                                                            start: 81,
                                                            end: 82,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb126e2ab50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                    ),
                                                                    start: 87,
                                                                    end: 90,
                                                                    as_str(): "u64",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [
                                                            (
                                                                DoubleColonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb126e2ab50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                        ),
                                                                        start: 90,
                                                                        end: 92,
                                                                        as_str(): "::",
                                                                    },
                                                                },
                                                                PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb126e2ab50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                            ),
                                                                            start: 92,
                                                                            end: 102,
                                                                            as_str(): "binary_xor",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                            ),
                                                        ],
                                                        incomplete_suffix: false,
                                                    },
                                                ),
                                                args: Parens {
                                                    inner: Punctuated {
                                                        value_separator_pairs: [
                                                            (
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb126e2ab50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 104,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                ),
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb126e2ab50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                        ),
                                                                        start: 104,
                                                                        end: 105,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: Some(
                                                            Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb126e2ab50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                                                ),
                                                                                start: 106,
                                                                                end: 107,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb126e2ab50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                                        ),
                                                        start: 102,
                                                        end: 108,
                                                        as_str(): "(a, b)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb126e2ab50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQl0TFI/builtin_type_method_call/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 110,
                                        as_str(): "{\n    let a = 1u64;\n    let b = 2u64;\n    u64::binary_xor(a, b)\n}",
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
