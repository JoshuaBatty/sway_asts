Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0f57bed10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0f57bed10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
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
                                        src (ptr): 0x00007fb0f57bed10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
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
                                            src (ptr): 0x00007fb0f57bed10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0f57bed10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 24,
                                                                    end: 28,
                                                                    as_str(): "b512",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 28,
                                                                    end: 30,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 30,
                                                                        end: 34,
                                                                        as_str(): "B512",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 40,
                                                                    end: 46,
                                                                    as_str(): "assert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 46,
                                                                    end: 48,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 48,
                                                                        end: 54,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 54,
                                                                end: 55,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    Path {
                                                        prefix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 60,
                                                                end: 67,
                                                                as_str(): "logging",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 69,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 69,
                                                                    end: 72,
                                                                    as_str(): "log",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb0f57bed10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 74,
                                                as_str(): "{\n    b512::B512,\n    assert::assert,\n    logging::log\n}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0f57bed10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                        ),
                                        start: 74,
                                        end: 75,
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
                                            src (ptr): 0x00007fb0f57bed10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                            ),
                                            start: 77,
                                            end: 79,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0f57bed10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                            ),
                                            start: 80,
                                            end: 84,
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
                                            src (ptr): 0x00007fb0f57bed10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                            ),
                                            start: 84,
                                            end: 86,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f57bed10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                    ),
                                                    start: 87,
                                                    end: 89,
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
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 94,
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
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 104,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 112,
                                                                as_str(): "hi_bits",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 112,
                                                                    end: 113,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 114,
                                                                                end: 118,
                                                                                as_str(): "b256",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 120,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 121,
                                                                    end: 187,
                                                                    as_str(): "0x7777777777777777777777777777777777777777777777777777777777777777",
                                                                },
                                                                parsed: 54036308310747557864333126337387690331525992843965596551746872537026127165303,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 187,
                                                            end: 188,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 193,
                                                            end: 196,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 204,
                                                                as_str(): "lo_bits",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 204,
                                                                    end: 205,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 206,
                                                                                end: 210,
                                                                                as_str(): "b256",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 211,
                                                            end: 212,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 213,
                                                                    end: 279,
                                                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 279,
                                                            end: 280,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 289,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 290,
                                                                end: 291,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 292,
                                                            end: 293,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FuncApp {
                                                        func: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 294,
                                                                            end: 298,
                                                                            as_str(): "B512",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 298,
                                                                                end: 300,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 300,
                                                                                    end: 304,
                                                                                    as_str(): "from",
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
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    Tuple(
                                                                        Parens {
                                                                            inner: Cons {
                                                                                head: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                    ),
                                                                                                    start: 306,
                                                                                                    end: 313,
                                                                                                    as_str(): "hi_bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [],
                                                                                        incomplete_suffix: false,
                                                                                    },
                                                                                ),
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                        ),
                                                                                        start: 313,
                                                                                        end: 314,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                                tail: Punctuated {
                                                                                    value_separator_pairs: [],
                                                                                    final_value_opt: Some(
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                            ),
                                                                                                            start: 315,
                                                                                                            end: 322,
                                                                                                            as_str(): "lo_bits",
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
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 305,
                                                                                end: 323,
                                                                                as_str(): "(hi_bits, lo_bits)",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 304,
                                                                end: 324,
                                                                as_str(): "((hi_bits, lo_bits))",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 324,
                                                            end: 325,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 330,
                                                            end: 333,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 334,
                                                                end: 341,
                                                                as_str(): "other_b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 342,
                                                            end: 343,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FuncApp {
                                                        func: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 348,
                                                                            as_str(): "B512",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                ),
                                                                                start: 348,
                                                                                end: 350,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 350,
                                                                                    end: 353,
                                                                                    as_str(): "new",
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
                                                                value_separator_pairs: [],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f57bed10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                ),
                                                                start: 353,
                                                                end: 355,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 355,
                                                            end: 356,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            LogicalAnd {
                                                lhs: Parens(
                                                    Parens {
                                                        inner: NotEqual {
                                                            lhs: Index {
                                                                target: Parens(
                                                                    Parens {
                                                                        inner: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 363,
                                                                                                end: 364,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 364,
                                                                                    end: 365,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 365,
                                                                                    end: 370,
                                                                                    as_str(): "bytes",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 362,
                                                                            end: 371,
                                                                            as_str(): "(b.bytes)",
                                                                        },
                                                                    },
                                                                ),
                                                                arg: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 372,
                                                                                    end: 373,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 371,
                                                                        end: 374,
                                                                        as_str(): "[0]",
                                                                    },
                                                                },
                                                            },
                                                            bang_eq_token: BangEqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 375,
                                                                    end: 377,
                                                                    as_str(): "!=",
                                                                },
                                                            },
                                                            rhs: Index {
                                                                target: Parens(
                                                                    Parens {
                                                                        inner: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 379,
                                                                                                end: 386,
                                                                                                as_str(): "other_b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [],
                                                                                    incomplete_suffix: false,
                                                                                },
                                                                            ),
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 386,
                                                                                    end: 387,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 387,
                                                                                    end: 392,
                                                                                    as_str(): "bytes",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 378,
                                                                            end: 393,
                                                                            as_str(): "(other_b.bytes)",
                                                                        },
                                                                    },
                                                                ),
                                                                arg: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 395,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 393,
                                                                        end: 396,
                                                                        as_str(): "[0]",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 361,
                                                            end: 397,
                                                            as_str(): "((b.bytes)[0] != (other_b.bytes)[0])",
                                                        },
                                                    },
                                                ),
                                                double_ampersand_token: DoubleAmpersandToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f57bed10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                        ),
                                                        start: 398,
                                                        end: 400,
                                                        as_str(): "&&",
                                                    },
                                                },
                                                rhs: Parens(
                                                    Parens {
                                                        inner: Equal {
                                                            lhs: Index {
                                                                target: Parens(
                                                                    Parens {
                                                                        inner: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 403,
                                                                                                end: 404,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 404,
                                                                                    end: 405,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 405,
                                                                                    end: 410,
                                                                                    as_str(): "bytes",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 402,
                                                                            end: 411,
                                                                            as_str(): "(b.bytes)",
                                                                        },
                                                                    },
                                                                ),
                                                                arg: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 412,
                                                                                    end: 413,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 411,
                                                                        end: 414,
                                                                        as_str(): "[1]",
                                                                    },
                                                                },
                                                            },
                                                            double_eq_token: DoubleEqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                    ),
                                                                    start: 415,
                                                                    end: 417,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            rhs: Index {
                                                                target: Parens(
                                                                    Parens {
                                                                        inner: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f57bed10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                                ),
                                                                                                start: 419,
                                                                                                end: 426,
                                                                                                as_str(): "other_b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [],
                                                                                    incomplete_suffix: false,
                                                                                },
                                                                            ),
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 426,
                                                                                    end: 427,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 427,
                                                                                    end: 432,
                                                                                    as_str(): "bytes",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f57bed10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                            ),
                                                                            start: 418,
                                                                            end: 433,
                                                                            as_str(): "(other_b.bytes)",
                                                                        },
                                                                    },
                                                                ),
                                                                arg: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f57bed10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                                    ),
                                                                                    start: 434,
                                                                                    end: 435,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f57bed10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                                        ),
                                                                        start: 433,
                                                                        end: 436,
                                                                        as_str(): "[1]",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f57bed10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                                            ),
                                                            start: 401,
                                                            end: 437,
                                                            as_str(): "((b.bytes)[1] == (other_b.bytes)[1])",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0f57bed10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlGclGV/empty_method_initializer/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 439,
                                        as_str(): "{\n    let hi_bits: b256 = 0x7777777777777777777777777777777777777777777777777777777777777777;\n    let lo_bits: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;\n\n    let b = B512::from((hi_bits, lo_bits));\n    let other_b = B512::new();\n    ((b.bytes)[0] != (other_b.bytes)[0]) && ((b.bytes)[1] == (other_b.bytes)[1])\n}",
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
