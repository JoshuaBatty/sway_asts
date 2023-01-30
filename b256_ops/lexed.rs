Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb14cbeadf0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb14cbeadf0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
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
                                        src (ptr): 0x00007fb14cbeadf0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
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
                                            src (ptr): 0x00007fb14cbeadf0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb14cbeadf0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb14cbeadf0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 22,
                                                as_str(): "hash",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb14cbeadf0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                ),
                                                start: 22,
                                                end: 24,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Group {
                                            imports: Braces {
                                                inner: Punctuated {
                                                    value_separator_pairs: [
                                                        (
                                                            Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                        ),
                                                                        start: 25,
                                                                        end: 34,
                                                                        as_str(): "keccak256",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 34,
                                                                    end: 35,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 36,
                                                                    end: 42,
                                                                    as_str(): "sha256",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    ),
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 43,
                                                    as_str(): "{keccak256, sha256}",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb14cbeadf0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 44,
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
                                            src (ptr): 0x00007fb14cbeadf0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                            ),
                                            start: 46,
                                            end: 48,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14cbeadf0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                            ),
                                            start: 49,
                                            end: 53,
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
                                            src (ptr): 0x00007fb14cbeadf0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                            ),
                                            start: 53,
                                            end: 55,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14cbeadf0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                    ),
                                                    start: 56,
                                                    end: 58,
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
                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                ),
                                                                start: 59,
                                                                end: 62,
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
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
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
                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 77,
                                                                as_str(): "aaaa",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 78,
                                                            end: 79,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 146,
                                                                    as_str(): "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                                                                },
                                                                parsed: 77194726158210796949047323339125271902179989777093709359638389338608753093290,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 147,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 152,
                                                            end: 155,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                ),
                                                                start: 156,
                                                                end: 160,
                                                                as_str(): "aaab",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 161,
                                                            end: 162,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 163,
                                                                    end: 230,
                                                                    as_str(): "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_b",
                                                                },
                                                                parsed: 77194726158210796949047323339125271902179989777093709359638389338608753093291,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 230,
                                                            end: 231,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 236,
                                                            end: 239,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                ),
                                                                start: 240,
                                                                end: 244,
                                                                as_str(): "abaa",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 245,
                                                            end: 246,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 247,
                                                                    end: 315,
                                                                    as_str(): "0xa_b_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                                                                },
                                                                parsed: 77647039006794063337420647499315459042231825654693867812917520526139663755946,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 315,
                                                            end: 316,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 323,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Equal {
                                                            lhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 324,
                                                                                end: 328,
                                                                                as_str(): "aaaa",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                            ),
                                                            double_eq_token: DoubleEqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 329,
                                                                    end: 331,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            rhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 332,
                                                                                end: 336,
                                                                                as_str(): "aaab",
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
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                ),
                                                                                start: 347,
                                                                                end: 348,
                                                                                as_str(): "0",
                                                                            },
                                                                            parsed: 0,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cbeadf0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                            ),
                                                            start: 337,
                                                            end: 354,
                                                            as_str(): "{\n        0\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                    ),
                                                                    start: 355,
                                                                    end: 359,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Continue(
                                                                IfExpr {
                                                                    if_token: IfToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                            ),
                                                                            start: 360,
                                                                            end: 362,
                                                                            as_str(): "if",
                                                                        },
                                                                    },
                                                                    condition: Expr(
                                                                        Equal {
                                                                            lhs: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 363,
                                                                                                end: 367,
                                                                                                as_str(): "aaaa",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [],
                                                                                    incomplete_suffix: false,
                                                                                },
                                                                            ),
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                    ),
                                                                                    start: 368,
                                                                                    end: 370,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 371,
                                                                                                end: 375,
                                                                                                as_str(): "abaa",
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
                                                                    ),
                                                                    then_block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: Some(
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                ),
                                                                                                start: 386,
                                                                                                end: 387,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 393,
                                                                            as_str(): "{\n        1\n    }",
                                                                        },
                                                                    },
                                                                    else_opt: Some(
                                                                        (
                                                                            ElseToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 398,
                                                                                    as_str(): "else",
                                                                                },
                                                                            },
                                                                            Continue(
                                                                                IfExpr {
                                                                                    if_token: IfToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                            ),
                                                                                            start: 399,
                                                                                            end: 401,
                                                                                            as_str(): "if",
                                                                                        },
                                                                                    },
                                                                                    condition: Expr(
                                                                                        Not {
                                                                                            bang_token: BangToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 402,
                                                                                                    end: 403,
                                                                                                    as_str(): "!",
                                                                                                },
                                                                                            },
                                                                                            expr: Parens(
                                                                                                Parens {
                                                                                                    inner: Equal {
                                                                                                        lhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 404,
                                                                                                                            end: 408,
                                                                                                                            as_str(): "aaaa",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                                incomplete_suffix: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        double_eq_token: DoubleEqToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 409,
                                                                                                                end: 411,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 412,
                                                                                                                            end: 416,
                                                                                                                            as_str(): "aaaa",
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
                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                        ),
                                                                                                        start: 403,
                                                                                                        end: 417,
                                                                                                        as_str(): "(aaaa == aaaa)",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                    then_block: Braces {
                                                                                        inner: CodeBlockContents {
                                                                                            statements: [],
                                                                                            final_expr_opt: Some(
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                ),
                                                                                                                start: 428,
                                                                                                                end: 429,
                                                                                                                as_str(): "2",
                                                                                                            },
                                                                                                            parsed: 2,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                            ),
                                                                                            start: 418,
                                                                                            end: 435,
                                                                                            as_str(): "{\n        2\n    }",
                                                                                        },
                                                                                    },
                                                                                    else_opt: Some(
                                                                                        (
                                                                                            ElseToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                    ),
                                                                                                    start: 436,
                                                                                                    end: 440,
                                                                                                    as_str(): "else",
                                                                                                },
                                                                                            },
                                                                                            Continue(
                                                                                                IfExpr {
                                                                                                    if_token: IfToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 441,
                                                                                                            end: 443,
                                                                                                            as_str(): "if",
                                                                                                        },
                                                                                                    },
                                                                                                    condition: Expr(
                                                                                                        Not {
                                                                                                            bang_token: BangToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 444,
                                                                                                                    end: 445,
                                                                                                                    as_str(): "!",
                                                                                                                },
                                                                                                            },
                                                                                                            expr: Parens(
                                                                                                                Parens {
                                                                                                                    inner: Equal {
                                                                                                                        lhs: FuncApp {
                                                                                                                            func: Path(
                                                                                                                                PathExpr {
                                                                                                                                    root_opt: None,
                                                                                                                                    prefix: PathExprSegment {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 446,
                                                                                                                                                end: 452,
                                                                                                                                                as_str(): "sha256",
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
                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 453,
                                                                                                                                                            end: 457,
                                                                                                                                                            as_str(): "aaaa",
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
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 452,
                                                                                                                                    end: 458,
                                                                                                                                    as_str(): "(aaaa)",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        },
                                                                                                                        double_eq_token: DoubleEqToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 459,
                                                                                                                                end: 461,
                                                                                                                                as_str(): "==",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        rhs: Literal(
                                                                                                                            Int(
                                                                                                                                LitInt {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 462,
                                                                                                                                        end: 528,
                                                                                                                                        as_str(): "0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e",
                                                                                                                                    },
                                                                                                                                    parsed: 101727063936689434835607961314781256938463604996053659903003450002731239770254,
                                                                                                                                    ty_opt: None,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 445,
                                                                                                                        end: 529,
                                                                                                                        as_str(): "(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e)",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                    ),
                                                                                                    then_block: Braces {
                                                                                                        inner: CodeBlockContents {
                                                                                                            statements: [],
                                                                                                            final_expr_opt: Some(
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 540,
                                                                                                                                end: 541,
                                                                                                                                as_str(): "3",
                                                                                                                            },
                                                                                                                            parsed: 3,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                            ),
                                                                                                            start: 530,
                                                                                                            end: 547,
                                                                                                            as_str(): "{\n        3\n    }",
                                                                                                        },
                                                                                                    },
                                                                                                    else_opt: Some(
                                                                                                        (
                                                                                                            ElseToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 548,
                                                                                                                    end: 552,
                                                                                                                    as_str(): "else",
                                                                                                                },
                                                                                                            },
                                                                                                            Continue(
                                                                                                                IfExpr {
                                                                                                                    if_token: IfToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 553,
                                                                                                                            end: 555,
                                                                                                                            as_str(): "if",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    condition: Expr(
                                                                                                                        Not {
                                                                                                                            bang_token: BangToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 556,
                                                                                                                                    end: 557,
                                                                                                                                    as_str(): "!",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            expr: Parens(
                                                                                                                                Parens {
                                                                                                                                    inner: Equal {
                                                                                                                                        lhs: FuncApp {
                                                                                                                                            func: Path(
                                                                                                                                                PathExpr {
                                                                                                                                                    root_opt: None,
                                                                                                                                                    prefix: PathExprSegment {
                                                                                                                                                        name: BaseIdent {
                                                                                                                                                            name_override_opt: None,
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 558,
                                                                                                                                                                end: 567,
                                                                                                                                                                as_str(): "keccak256",
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
                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 568,
                                                                                                                                                                            end: 572,
                                                                                                                                                                            as_str(): "aaaa",
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
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 567,
                                                                                                                                                    end: 573,
                                                                                                                                                    as_str(): "(aaaa)",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        double_eq_token: DoubleEqToken {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 574,
                                                                                                                                                end: 576,
                                                                                                                                                as_str(): "==",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        rhs: Literal(
                                                                                                                                            Int(
                                                                                                                                                LitInt {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 577,
                                                                                                                                                        end: 643,
                                                                                                                                                        as_str(): "0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90",
                                                                                                                                                    },
                                                                                                                                                    parsed: 14895508228889110621262984872494660806315753078798056959773222445314613636240,
                                                                                                                                                    ty_opt: None,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 557,
                                                                                                                                        end: 644,
                                                                                                                                        as_str(): "(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90)",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    then_block: Braces {
                                                                                                                        inner: CodeBlockContents {
                                                                                                                            statements: [],
                                                                                                                            final_expr_opt: Some(
                                                                                                                                Literal(
                                                                                                                                    Int(
                                                                                                                                        LitInt {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 655,
                                                                                                                                                end: 656,
                                                                                                                                                as_str(): "4",
                                                                                                                                            },
                                                                                                                                            parsed: 4,
                                                                                                                                            ty_opt: None,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 645,
                                                                                                                            end: 662,
                                                                                                                            as_str(): "{\n        4\n    }",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    else_opt: Some(
                                                                                                                        (
                                                                                                                            ElseToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 663,
                                                                                                                                    end: 667,
                                                                                                                                    as_str(): "else",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            Continue(
                                                                                                                                IfExpr {
                                                                                                                                    if_token: IfToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 668,
                                                                                                                                            end: 670,
                                                                                                                                            as_str(): "if",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    condition: Expr(
                                                                                                                                        Not {
                                                                                                                                            bang_token: BangToken {
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 671,
                                                                                                                                                    end: 672,
                                                                                                                                                    as_str(): "!",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            expr: Parens(
                                                                                                                                                Parens {
                                                                                                                                                    inner: Equal {
                                                                                                                                                        lhs: FuncApp {
                                                                                                                                                            func: Path(
                                                                                                                                                                PathExpr {
                                                                                                                                                                    root_opt: None,
                                                                                                                                                                    prefix: PathExprSegment {
                                                                                                                                                                        name: BaseIdent {
                                                                                                                                                                            name_override_opt: None,
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 673,
                                                                                                                                                                                end: 679,
                                                                                                                                                                                as_str(): "sha256",
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
                                                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                        path: Some(
                                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                        ),
                                                                                                                                                                                                        start: 681,
                                                                                                                                                                                                        end: 685,
                                                                                                                                                                                                        as_str(): "aaaa",
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
                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 685,
                                                                                                                                                                                            end: 686,
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
                                                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                                ),
                                                                                                                                                                                                                start: 687,
                                                                                                                                                                                                                end: 691,
                                                                                                                                                                                                                as_str(): "abaa",
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
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 680,
                                                                                                                                                                                    end: 692,
                                                                                                                                                                                    as_str(): "(aaaa, abaa)",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        ),
                                                                                                                                                                    ),
                                                                                                                                                                },
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 679,
                                                                                                                                                                    end: 693,
                                                                                                                                                                    as_str(): "((aaaa, abaa))",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                        double_eq_token: DoubleEqToken {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 694,
                                                                                                                                                                end: 696,
                                                                                                                                                                as_str(): "==",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                        rhs: Literal(
                                                                                                                                                            Int(
                                                                                                                                                                LitInt {
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 697,
                                                                                                                                                                        end: 763,
                                                                                                                                                                        as_str(): "0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce",
                                                                                                                                                                    },
                                                                                                                                                                    parsed: 74512640259888620028694275759594168228492722714493359294375808478096552760782,
                                                                                                                                                                    ty_opt: None,
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                    },
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 672,
                                                                                                                                                        end: 764,
                                                                                                                                                        as_str(): "(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce)",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    then_block: Braces {
                                                                                                                                        inner: CodeBlockContents {
                                                                                                                                            statements: [],
                                                                                                                                            final_expr_opt: Some(
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 775,
                                                                                                                                                                end: 776,
                                                                                                                                                                as_str(): "5",
                                                                                                                                                            },
                                                                                                                                                            parsed: 5,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        },
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 765,
                                                                                                                                            end: 782,
                                                                                                                                            as_str(): "{\n        5\n    }",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    else_opt: Some(
                                                                                                                                        (
                                                                                                                                            ElseToken {
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 783,
                                                                                                                                                    end: 787,
                                                                                                                                                    as_str(): "else",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            Continue(
                                                                                                                                                IfExpr {
                                                                                                                                                    if_token: IfToken {
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 788,
                                                                                                                                                            end: 790,
                                                                                                                                                            as_str(): "if",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    condition: Expr(
                                                                                                                                                        Not {
                                                                                                                                                            bang_token: BangToken {
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 791,
                                                                                                                                                                    end: 792,
                                                                                                                                                                    as_str(): "!",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            expr: Parens(
                                                                                                                                                                Parens {
                                                                                                                                                                    inner: Equal {
                                                                                                                                                                        lhs: FuncApp {
                                                                                                                                                                            func: Path(
                                                                                                                                                                                PathExpr {
                                                                                                                                                                                    root_opt: None,
                                                                                                                                                                                    prefix: PathExprSegment {
                                                                                                                                                                                        name: BaseIdent {
                                                                                                                                                                                            name_override_opt: None,
                                                                                                                                                                                            span: Span {
                                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                ),
                                                                                                                                                                                                start: 793,
                                                                                                                                                                                                end: 802,
                                                                                                                                                                                                as_str(): "keccak256",
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
                                                                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                                        path: Some(
                                                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                                        ),
                                                                                                                                                                                                                        start: 804,
                                                                                                                                                                                                                        end: 808,
                                                                                                                                                                                                                        as_str(): "aaaa",
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
                                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                            ),
                                                                                                                                                                                                            start: 808,
                                                                                                                                                                                                            end: 809,
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
                                                                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                                                ),
                                                                                                                                                                                                                                start: 810,
                                                                                                                                                                                                                                end: 814,
                                                                                                                                                                                                                                as_str(): "abaa",
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
                                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                                    path: Some(
                                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                                    ),
                                                                                                                                                                                                    start: 803,
                                                                                                                                                                                                    end: 815,
                                                                                                                                                                                                    as_str(): "(aaaa, abaa)",
                                                                                                                                                                                                },
                                                                                                                                                                                            },
                                                                                                                                                                                        ),
                                                                                                                                                                                    ),
                                                                                                                                                                                },
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 802,
                                                                                                                                                                                    end: 816,
                                                                                                                                                                                    as_str(): "((aaaa, abaa))",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                        double_eq_token: DoubleEqToken {
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 817,
                                                                                                                                                                                end: 819,
                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                        rhs: Literal(
                                                                                                                                                                            Int(
                                                                                                                                                                                LitInt {
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 820,
                                                                                                                                                                                        end: 886,
                                                                                                                                                                                        as_str(): "0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd",
                                                                                                                                                                                    },
                                                                                                                                                                                    parsed: 36097307807773068490274079792434907543558243402439323594886960998077905519293,
                                                                                                                                                                                    ty_opt: None,
                                                                                                                                                                                },
                                                                                                                                                                            ),
                                                                                                                                                                        ),
                                                                                                                                                                    },
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 792,
                                                                                                                                                                        end: 887,
                                                                                                                                                                        as_str(): "(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd)",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    then_block: Braces {
                                                                                                                                                        inner: CodeBlockContents {
                                                                                                                                                            statements: [],
                                                                                                                                                            final_expr_opt: Some(
                                                                                                                                                                Literal(
                                                                                                                                                                    Int(
                                                                                                                                                                        LitInt {
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 898,
                                                                                                                                                                                end: 899,
                                                                                                                                                                                as_str(): "6",
                                                                                                                                                                            },
                                                                                                                                                                            parsed: 6,
                                                                                                                                                                            ty_opt: None,
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                        },
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 888,
                                                                                                                                                            end: 905,
                                                                                                                                                            as_str(): "{\n        6\n    }",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    else_opt: Some(
                                                                                                                                                        (
                                                                                                                                                            ElseToken {
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 906,
                                                                                                                                                                    end: 910,
                                                                                                                                                                    as_str(): "else",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            Break(
                                                                                                                                                                Braces {
                                                                                                                                                                    inner: CodeBlockContents {
                                                                                                                                                                        statements: [],
                                                                                                                                                                        final_expr_opt: Some(
                                                                                                                                                                            Literal(
                                                                                                                                                                                Int(
                                                                                                                                                                                    LitInt {
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 921,
                                                                                                                                                                                            end: 924,
                                                                                                                                                                                            as_str(): "100",
                                                                                                                                                                                        },
                                                                                                                                                                                        parsed: 100,
                                                                                                                                                                                        ty_opt: None,
                                                                                                                                                                                    },
                                                                                                                                                                                ),
                                                                                                                                                                            ),
                                                                                                                                                                        ),
                                                                                                                                                                    },
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb14cbeadf0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 911,
                                                                                                                                                                        end: 930,
                                                                                                                                                                        as_str(): "{\n        100\n    }",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                    ),
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14cbeadf0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIREPOfvl/b256_ops/src/main.sw",
                                        ),
                                        start: 63,
                                        end: 932,
                                        as_str(): "{\n    let aaaa = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    let aaab = 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_b;\n    let abaa = 0xa_b_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;\n    if aaaa == aaab {\n        0\n    } else if aaaa == abaa {\n        1\n    } else if !(aaaa == aaaa) {\n        2\n    } else if !(sha256(aaaa) == 0xe0e77a507412b120f6ede61f62295b1a7b2ff19d3dcc8f7253e51663470c888e) {\n        3\n    } else if !(keccak256(aaaa) == 0x20ee8f1366f06926e9e8771d8fb9007a8537c8dfdb6a3f8c2cfd64db19d2ec90) {\n        4\n    } else if !(sha256((aaaa, abaa)) == 0xa4bca8eb8f338f7fda26960fa43bfe34fbc562e2ee0d7c6e8856c1c587f215ce) {\n        5\n    } else if !(keccak256((aaaa, abaa)) == 0x4fce5a297040d82eecf7b0ae4855ad43698f191ee38820e27748648765bc42bd) {\n        6\n    } else {\n        100\n    }\n}",
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
