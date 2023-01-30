Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a1239c1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a1239c1f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                        src (ptr): 0x00007f8a1239c1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                            src (ptr): 0x00007f8a1239c1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 15,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a1239c1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                            ),
                                            start: 15,
                                            end: 17,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007f8a1239c1f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                        src (ptr): 0x00007f8a1239c1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                            src (ptr): 0x00007f8a1239c1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                            ),
                                            start: 21,
                                            end: 23,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1239c1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                            src (ptr): 0x00007f8a1239c1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                    src (ptr): 0x00007f8a1239c1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 76,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Not {
                                                        bang_token: BangToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                            src (ptr): 0x00007f8a1239c1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
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
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 101,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Not {
                                                        bang_token: BangToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        expr: Not {
                                                            bang_token: BangToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 105,
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
                                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                ),
                                                                                start: 105,
                                                                                end: 106,
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
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                            ),
                                                            start: 106,
                                                            end: 107,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                            ),
                                                            start: 121,
                                                            end: 124,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                ),
                                                                start: 125,
                                                                end: 126,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                            ),
                                                            start: 127,
                                                            end: 128,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Not {
                                                        bang_token: BangToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 130,
                                                                as_str(): "!",
                                                            },
                                                        },
                                                        expr: Not {
                                                            bang_token: BangToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                    ),
                                                                    start: 131,
                                                                    end: 132,
                                                                    as_str(): "!",
                                                                },
                                                            },
                                                            expr: Not {
                                                                bang_token: BangToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1239c1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                        ),
                                                                        start: 133,
                                                                        end: 134,
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
                                                                                    src (ptr): 0x00007f8a1239c1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
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
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1239c1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 136,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1239c1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                                                ),
                                                                start: 149,
                                                                end: 150,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a1239c1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRS5HBuP/unary_not_basic/src/main.sw",
                                        ),
                                        start: 39,
                                        end: 152,
                                        as_str(): "{\n    let a: bool = true;\n    let b = !a; // false\n    let c = ! !b; // false\n    let d = ! ! !c; // true\n    d\n}",
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
