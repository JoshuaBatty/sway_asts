Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a158f2440,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a158f2440,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
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
                                        src (ptr): 0x00007f8a158f2440,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                        ),
                                        start: 8,
                                        end: 11,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 15,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 15,
                                            end: 17,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007f8a158f2440,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                ),
                                                start: 17,
                                                end: 18,
                                                as_str(): "*",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a158f2440,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 19,
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
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 21,
                                            end: 23,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 24,
                                            end: 28,
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
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 28,
                                            end: 30,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 31,
                                                    end: 33,
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
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 38,
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
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 45,
                                                            end: 48,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 49,
                                                                end: 50,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 50,
                                                                    end: 51,
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
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 52,
                                                                                end: 56,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 58,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 59,
                                                                    end: 63,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
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
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
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
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
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
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 76,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Not {
                                                        bang_token: BangToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 78,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        expr: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 79,
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
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 79,
                                                            end: 80,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 97,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 99,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 101,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Not {
                                                        bang_token: BangToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        expr: Parens(
                                                            Parens {
                                                                inner: Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 105,
                                                                            end: 106,
                                                                            as_str(): "!",
                                                                        },
                                                                    },
                                                                    expr: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a158f2440,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
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
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 103,
                                                                    end: 108,
                                                                    as_str(): "( !b)",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 109,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 123,
                                                            end: 126,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 127,
                                                                end: 128,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 129,
                                                            end: 130,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Not {
                                                        bang_token: BangToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 131,
                                                                end: 132,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        expr: Parens(
                                                            Parens {
                                                                inner: Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 134,
                                                                            end: 135,
                                                                            as_str(): "!",
                                                                        },
                                                                    },
                                                                    expr: Not {
                                                                        bang_token: BangToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 136,
                                                                                end: 137,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        expr: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a158f2440,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                            ),
                                                                                            start: 137,
                                                                                            end: 138,
                                                                                            as_str(): "c",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 132,
                                                                    end: 139,
                                                                    as_str(): "( ! !c)",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 139,
                                                            end: 140,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 153,
                                                            end: 156,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 158,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 159,
                                                            end: 160,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Parens(
                                                        Parens {
                                                            inner: Not {
                                                                bang_token: BangToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a158f2440,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                        ),
                                                                        start: 163,
                                                                        end: 164,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                expr: Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 165,
                                                                            end: 166,
                                                                            as_str(): "!",
                                                                        },
                                                                    },
                                                                    expr: Not {
                                                                        bang_token: BangToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 167,
                                                                                end: 168,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        expr: Parens(
                                                                            Parens {
                                                                                inner: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                    ),
                                                                                                    start: 169,
                                                                                                    end: 170,
                                                                                                    as_str(): "d",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [],
                                                                                        incomplete_suffix: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 168,
                                                                                    end: 171,
                                                                                    as_str(): "(d)",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 161,
                                                                end: 172,
                                                                as_str(): "( ! ! !(d))",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 173,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            LogicalOr {
                                                lhs: Not {
                                                    bang_token: BangToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 178,
                                                            end: 179,
                                                            as_str(): "!",
                                                        },
                                                    },
                                                    expr: Parens(
                                                        Parens {
                                                            inner: FuncApp {
                                                                func: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a158f2440,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                    ),
                                                                                    start: 180,
                                                                                    end: 188,
                                                                                    as_str(): "and_true",
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
                                                                        final_value_opt: Some(
                                                                            Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                                ),
                                                                                                start: 189,
                                                                                                end: 190,
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
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a158f2440,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                        ),
                                                                        start: 188,
                                                                        end: 191,
                                                                        as_str(): "(a)",
                                                                    },
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 192,
                                                                as_str(): "(and_true(a))",
                                                            },
                                                        },
                                                    ),
                                                },
                                                double_pipe_token: DoublePipeToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a158f2440,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                        ),
                                                        start: 193,
                                                        end: 195,
                                                        as_str(): "||",
                                                    },
                                                },
                                                rhs: Literal(
                                                    Bool(
                                                        LitBool {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 200,
                                                                as_str(): "true",
                                                            },
                                                            kind: True,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a158f2440,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                        ),
                                        start: 39,
                                        end: 202,
                                        as_str(): "{\n    let a: bool = true;\n    let b = !a; // false\n    let c = !( !b); // false\n    let d = !( ! !c); // true\n    let e = ( ! ! !(d));\n    !(and_true(a)) || true\n}",
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
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 204,
                                            end: 206,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 207,
                                            end: 215,
                                            as_str(): "and_true",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 216,
                                                                    end: 217,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 217,
                                                                end: 218,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a158f2440,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                            ),
                                                                            start: 219,
                                                                            end: 223,
                                                                            as_str(): "bool",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a158f2440,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                            ),
                                            start: 215,
                                            end: 224,
                                            as_str(): "(x: bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a158f2440,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                    ),
                                                    start: 225,
                                                    end: 227,
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
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 228,
                                                                end: 232,
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
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 239,
                                                            end: 242,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 243,
                                                                end: 244,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 245,
                                                            end: 246,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Not {
                                                        bang_token: BangToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 247,
                                                                end: 248,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        expr: Not {
                                                            bang_token: BangToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 249,
                                                                    end: 250,
                                                                    as_str(): "!",
                                                                },
                                                            },
                                                            expr: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a158f2440,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                                ),
                                                                                start: 250,
                                                                                end: 251,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a158f2440,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                            ),
                                                            start: 251,
                                                            end: 252,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            LogicalAnd {
                                                lhs: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a158f2440,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                    ),
                                                                    start: 257,
                                                                    end: 258,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                        incomplete_suffix: false,
                                                    },
                                                ),
                                                double_ampersand_token: DoubleAmpersandToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a158f2440,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                        ),
                                                        start: 259,
                                                        end: 261,
                                                        as_str(): "&&",
                                                    },
                                                },
                                                rhs: Literal(
                                                    Bool(
                                                        LitBool {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a158f2440,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                                                ),
                                                                start: 262,
                                                                end: 266,
                                                                as_str(): "true",
                                                            },
                                                            kind: True,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a158f2440,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRxVncXk/unary_not_basic_2/src/main.sw",
                                        ),
                                        start: 233,
                                        end: 268,
                                        as_str(): "{\n    let y = ! !x;\n    x && true\n}",
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
