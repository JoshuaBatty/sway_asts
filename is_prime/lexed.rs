Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fd209f60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fd209f60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 17,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 17,
                                            end: 19,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fd209f60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 20,
                                                as_str(): "*",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 20,
                                        end: 21,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 22,
                                        end: 25,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 26,
                                            end: 29,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 29,
                                            end: 31,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd209f60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                ),
                                                start: 31,
                                                end: 37,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fd209f60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                ),
                                                start: 37,
                                                end: 39,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd209f60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                    ),
                                                    start: 39,
                                                    end: 45,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 46,
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 50,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 51,
                                            end: 62,
                                            as_str(): "check_prime",
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
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 64,
                                                                    as_str(): "n",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
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
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 66,
                                                                            end: 69,
                                                                            as_str(): "u64",
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 62,
                                            end: 70,
                                            as_str(): "(n: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd209f60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                    ),
                                                    start: 71,
                                                    end: 73,
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 78,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 85,
                                                            end: 87,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        LogicalOr {
                                                            lhs: Equal {
                                                                lhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 88,
                                                                                    end: 89,
                                                                                    as_str(): "n",
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
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 90,
                                                                        end: 92,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 93,
                                                                                end: 94,
                                                                                as_str(): "0",
                                                                            },
                                                                            parsed: 0,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            double_pipe_token: DoublePipeToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 97,
                                                                    as_str(): "||",
                                                                },
                                                            },
                                                            rhs: Equal {
                                                                lhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 98,
                                                                                    end: 99,
                                                                                    as_str(): "n",
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
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 100,
                                                                        end: 102,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 103,
                                                                                end: 104,
                                                                                as_str(): "1",
                                                                            },
                                                                            parsed: 1,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 115,
                                                                                end: 120,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 105,
                                                            end: 126,
                                                            as_str(): "{\n        false\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                    ),
                                                                    start: 127,
                                                                    end: 131,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [
                                                                            Let(
                                                                                StatementLet {
                                                                                    let_token: LetToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 142,
                                                                                            end: 145,
                                                                                            as_str(): "let",
                                                                                        },
                                                                                    },
                                                                                    pattern: Var {
                                                                                        reference: None,
                                                                                        mutable: Some(
                                                                                            MutToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 146,
                                                                                                    end: 149,
                                                                                                    as_str(): "mut",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 150,
                                                                                                end: 162,
                                                                                                as_str(): "is_not_prime",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    ty_opt: None,
                                                                                    eq_token: EqToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 163,
                                                                                            end: 164,
                                                                                            as_str(): "=",
                                                                                        },
                                                                                    },
                                                                                    expr: Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 165,
                                                                                                    end: 170,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    semicolon_token: SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 170,
                                                                                            end: 171,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            Let(
                                                                                StatementLet {
                                                                                    let_token: LetToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 180,
                                                                                            end: 183,
                                                                                            as_str(): "let",
                                                                                        },
                                                                                    },
                                                                                    pattern: Var {
                                                                                        reference: None,
                                                                                        mutable: Some(
                                                                                            MutToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 184,
                                                                                                    end: 187,
                                                                                                    as_str(): "mut",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 188,
                                                                                                end: 189,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    ty_opt: None,
                                                                                    eq_token: EqToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 190,
                                                                                            end: 191,
                                                                                            as_str(): "=",
                                                                                        },
                                                                                    },
                                                                                    expr: Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 192,
                                                                                                    end: 193,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                                parsed: 2,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    semicolon_token: SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 193,
                                                                                            end: 194,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            Expr {
                                                                                expr: While {
                                                                                    while_token: WhileToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 203,
                                                                                            end: 208,
                                                                                            as_str(): "while",
                                                                                        },
                                                                                    },
                                                                                    condition: LessThan {
                                                                                        lhs: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                            ),
                                                                                                            start: 209,
                                                                                                            end: 210,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                                suffix: [],
                                                                                                incomplete_suffix: false,
                                                                                            },
                                                                                        ),
                                                                                        less_than_token: LessThanToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                ),
                                                                                                start: 211,
                                                                                                end: 212,
                                                                                                as_str(): "<",
                                                                                            },
                                                                                        },
                                                                                        rhs: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                            ),
                                                                                                            start: 213,
                                                                                                            end: 214,
                                                                                                            as_str(): "n",
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
                                                                                    block: Braces {
                                                                                        inner: CodeBlockContents {
                                                                                            statements: [
                                                                                                Expr {
                                                                                                    expr: If(
                                                                                                        IfExpr {
                                                                                                            if_token: IfToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 230,
                                                                                                                    end: 232,
                                                                                                                    as_str(): "if",
                                                                                                                },
                                                                                                            },
                                                                                                            condition: Expr(
                                                                                                                Equal {
                                                                                                                    lhs: Modulo {
                                                                                                                        lhs: Path(
                                                                                                                            PathExpr {
                                                                                                                                root_opt: None,
                                                                                                                                prefix: PathExprSegment {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 233,
                                                                                                                                            end: 234,
                                                                                                                                            as_str(): "n",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    generics_opt: None,
                                                                                                                                },
                                                                                                                                suffix: [],
                                                                                                                                incomplete_suffix: false,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        percent_token: PercentToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 235,
                                                                                                                                end: 236,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        rhs: Path(
                                                                                                                            PathExpr {
                                                                                                                                root_opt: None,
                                                                                                                                prefix: PathExprSegment {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 237,
                                                                                                                                            end: 238,
                                                                                                                                            as_str(): "i",
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
                                                                                                                    double_eq_token: DoubleEqToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 239,
                                                                                                                            end: 241,
                                                                                                                            as_str(): "==",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    rhs: Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 242,
                                                                                                                                    end: 243,
                                                                                                                                    as_str(): "0",
                                                                                                                                },
                                                                                                                                parsed: 0,
                                                                                                                                ty_opt: None,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                },
                                                                                                            ),
                                                                                                            then_block: Braces {
                                                                                                                inner: CodeBlockContents {
                                                                                                                    statements: [
                                                                                                                        Expr {
                                                                                                                            expr: Reassignment {
                                                                                                                                assignable: Var(
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 262,
                                                                                                                                            end: 274,
                                                                                                                                            as_str(): "is_not_prime",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                reassignment_op: ReassignmentOp {
                                                                                                                                    variant: Equals,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 275,
                                                                                                                                        end: 276,
                                                                                                                                        as_str(): "=",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                expr: Literal(
                                                                                                                                    Bool(
                                                                                                                                        LitBool {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 277,
                                                                                                                                                end: 281,
                                                                                                                                                as_str(): "true",
                                                                                                                                            },
                                                                                                                                            kind: True,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            },
                                                                                                                            semicolon_token_opt: Some(
                                                                                                                                SemicolonToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 281,
                                                                                                                                        end: 282,
                                                                                                                                        as_str(): ";",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        Expr {
                                                                                                                            expr: Reassignment {
                                                                                                                                assignable: Var(
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 299,
                                                                                                                                            end: 300,
                                                                                                                                            as_str(): "i",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                reassignment_op: ReassignmentOp {
                                                                                                                                    variant: Equals,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 301,
                                                                                                                                        end: 302,
                                                                                                                                        as_str(): "=",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                expr: Path(
                                                                                                                                    PathExpr {
                                                                                                                                        root_opt: None,
                                                                                                                                        prefix: PathExprSegment {
                                                                                                                                            name: BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 303,
                                                                                                                                                    end: 304,
                                                                                                                                                    as_str(): "n",
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
                                                                                                                            semicolon_token_opt: Some(
                                                                                                                                SemicolonToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 304,
                                                                                                                                        end: 305,
                                                                                                                                        as_str(): ";",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    final_expr_opt: None,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 244,
                                                                                                                    end: 328,
                                                                                                                    as_str(): "{\n                is_not_prime = true;\n                i = n; // break\n            }",
                                                                                                                },
                                                                                                            },
                                                                                                            else_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                    semicolon_token_opt: Some(
                                                                                                        SemicolonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 328,
                                                                                                                end: 329,
                                                                                                                as_str(): ";",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                Expr {
                                                                                                    expr: Reassignment {
                                                                                                        assignable: Var(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 342,
                                                                                                                    end: 343,
                                                                                                                    as_str(): "i",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        reassignment_op: ReassignmentOp {
                                                                                                            variant: Equals,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 344,
                                                                                                                end: 345,
                                                                                                                as_str(): "=",
                                                                                                            },
                                                                                                        },
                                                                                                        expr: Add {
                                                                                                            lhs: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 346,
                                                                                                                                end: 347,
                                                                                                                                as_str(): "i",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            add_token: AddToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 348,
                                                                                                                    end: 349,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                            },
                                                                                                            rhs: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 350,
                                                                                                                            end: 351,
                                                                                                                            as_str(): "1",
                                                                                                                        },
                                                                                                                        parsed: 1,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                    },
                                                                                                    semicolon_token_opt: Some(
                                                                                                        SemicolonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                                ),
                                                                                                                start: 351,
                                                                                                                end: 352,
                                                                                                                as_str(): ";",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                            ],
                                                                                            final_expr_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 216,
                                                                                            end: 362,
                                                                                            as_str(): "{\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                semicolon_token_opt: None,
                                                                            },
                                                                        ],
                                                                        final_expr_opt: Some(
                                                                            Not {
                                                                                bang_token: BangToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                        ),
                                                                                        start: 372,
                                                                                        end: 373,
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
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 373,
                                                                                                    end: 385,
                                                                                                    as_str(): "is_not_prime",
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
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 132,
                                                                        end: 391,
                                                                        as_str(): "{\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 79,
                                        end: 393,
                                        as_str(): "{\n    if n == 0 || n == 1 {\n        false\n    } else {\n        let mut is_not_prime = false;\n        let mut i = 2;\n        while i < n  {\n            if n % i == 0 {\n                is_not_prime = true;\n                i = n; // break\n            };\n            i = i + 1;\n        }\n\n        !is_not_prime\n    }\n}",
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 395,
                                            end: 397,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 398,
                                            end: 402,
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
                                            src (ptr): 0x00007fe0fd209f60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                            ),
                                            start: 402,
                                            end: 404,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd209f60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                    ),
                                                    start: 405,
                                                    end: 407,
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
                                                                src (ptr): 0x00007fe0fd209f60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                ),
                                                                start: 408,
                                                                end: 412,
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
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 419,
                                                                        end: 425,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 426,
                                                                                            end: 437,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 438,
                                                                                                    end: 440,
                                                                                                    as_str(): "64",
                                                                                                },
                                                                                                parsed: 64,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 437,
                                                                                end: 441,
                                                                                as_str(): "(64)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 442,
                                                                            end: 444,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 445,
                                                                                    end: 450,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 425,
                                                            end: 451,
                                                            as_str(): "(check_prime(64) == false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 451,
                                                            end: 452,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 457,
                                                                        end: 463,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 464,
                                                                                            end: 475,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 476,
                                                                                                    end: 477,
                                                                                                    as_str(): "8",
                                                                                                },
                                                                                                parsed: 8,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 475,
                                                                                end: 478,
                                                                                as_str(): "(8)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 479,
                                                                            end: 481,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 482,
                                                                                    end: 487,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 463,
                                                            end: 488,
                                                            as_str(): "(check_prime(8) == false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 488,
                                                            end: 489,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 494,
                                                                        end: 500,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 501,
                                                                                            end: 512,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 513,
                                                                                                    end: 514,
                                                                                                    as_str(): "7",
                                                                                                },
                                                                                                parsed: 7,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 512,
                                                                                end: 515,
                                                                                as_str(): "(7)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 516,
                                                                            end: 518,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 519,
                                                                                    end: 523,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 500,
                                                            end: 524,
                                                            as_str(): "(check_prime(7) == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 524,
                                                            end: 525,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 530,
                                                                        end: 536,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 537,
                                                                                            end: 548,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 549,
                                                                                                    end: 551,
                                                                                                    as_str(): "11",
                                                                                                },
                                                                                                parsed: 11,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 548,
                                                                                end: 552,
                                                                                as_str(): "(11)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 553,
                                                                            end: 555,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 556,
                                                                                    end: 560,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 536,
                                                            end: 561,
                                                            as_str(): "(check_prime(11) == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 561,
                                                            end: 562,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 567,
                                                                        end: 573,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 574,
                                                                                            end: 585,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 586,
                                                                                                    end: 588,
                                                                                                    as_str(): "13",
                                                                                                },
                                                                                                parsed: 13,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 585,
                                                                                end: 589,
                                                                                as_str(): "(13)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 590,
                                                                            end: 592,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 593,
                                                                                    end: 597,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 573,
                                                            end: 598,
                                                            as_str(): "(check_prime(13) == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 598,
                                                            end: 599,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 604,
                                                                        end: 610,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 611,
                                                                                            end: 622,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 623,
                                                                                                    end: 624,
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
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 622,
                                                                                end: 625,
                                                                                as_str(): "(2)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 626,
                                                                            end: 628,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 629,
                                                                                    end: 633,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 610,
                                                            end: 634,
                                                            as_str(): "(check_prime(2) == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 634,
                                                            end: 635,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 640,
                                                                        end: 646,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 647,
                                                                                            end: 658,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 659,
                                                                                                    end: 660,
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
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 658,
                                                                                end: 661,
                                                                                as_str(): "(3)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 662,
                                                                            end: 664,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 665,
                                                                                    end: 669,
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
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 646,
                                                            end: 670,
                                                            as_str(): "(check_prime(3) == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 670,
                                                            end: 671,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 676,
                                                                        end: 682,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 683,
                                                                                            end: 694,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 695,
                                                                                                    end: 696,
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
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 694,
                                                                                end: 697,
                                                                                as_str(): "(1)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 698,
                                                                            end: 700,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 701,
                                                                                    end: 706,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 682,
                                                            end: 707,
                                                            as_str(): "(check_prime(1) == false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 707,
                                                            end: 708,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 713,
                                                                        end: 719,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 720,
                                                                                            end: 731,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 732,
                                                                                                    end: 733,
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
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 731,
                                                                                end: 734,
                                                                                as_str(): "(0)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 735,
                                                                            end: 737,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 738,
                                                                                    end: 743,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 719,
                                                            end: 744,
                                                            as_str(): "(check_prime(0) == false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 744,
                                                            end: 745,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 751,
                                                                        end: 757,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 758,
                                                                                            end: 769,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 770,
                                                                                                    end: 772,
                                                                                                    as_str(): "11",
                                                                                                },
                                                                                                parsed: 11,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 769,
                                                                                end: 773,
                                                                                as_str(): "(11)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 774,
                                                                            end: 776,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 777,
                                                                                            end: 788,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 789,
                                                                                                    end: 791,
                                                                                                    as_str(): "17",
                                                                                                },
                                                                                                parsed: 17,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 788,
                                                                                end: 792,
                                                                                as_str(): "(17)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 757,
                                                            end: 793,
                                                            as_str(): "(check_prime(11) == check_prime(17))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 793,
                                                            end: 794,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 799,
                                                                        end: 805,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 806,
                                                                                            end: 817,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 818,
                                                                                                    end: 820,
                                                                                                    as_str(): "12",
                                                                                                },
                                                                                                parsed: 12,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 817,
                                                                                end: 821,
                                                                                as_str(): "(12)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 822,
                                                                            end: 824,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 825,
                                                                                    end: 830,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 805,
                                                            end: 831,
                                                            as_str(): "(check_prime(12) == false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 831,
                                                            end: 832,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 837,
                                                                        end: 843,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 844,
                                                                                            end: 855,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 856,
                                                                                                    end: 858,
                                                                                                    as_str(): "18",
                                                                                                },
                                                                                                parsed: 18,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 855,
                                                                                end: 859,
                                                                                as_str(): "(18)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 860,
                                                                            end: 862,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                    ),
                                                                                    start: 863,
                                                                                    end: 868,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 843,
                                                            end: 869,
                                                            as_str(): "(check_prime(18) == false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 869,
                                                            end: 870,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd209f60,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                        ),
                                                                        start: 875,
                                                                        end: 881,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 882,
                                                                                            end: 893,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 894,
                                                                                                    end: 896,
                                                                                                    as_str(): "12",
                                                                                                },
                                                                                                parsed: 12,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 893,
                                                                                end: 897,
                                                                                as_str(): "(12)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                            ),
                                                                            start: 898,
                                                                            end: 900,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd209f60,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                            ),
                                                                                            start: 901,
                                                                                            end: 912,
                                                                                            as_str(): "check_prime",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd209f60,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                                    ),
                                                                                                    start: 913,
                                                                                                    end: 915,
                                                                                                    as_str(): "18",
                                                                                                },
                                                                                                parsed: 18,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd209f60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                                                ),
                                                                                start: 912,
                                                                                end: 916,
                                                                                as_str(): "(18)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 881,
                                                            end: 917,
                                                            as_str(): "(check_prime(12) == check_prime(18))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 917,
                                                            end: 918,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd209f60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                                            ),
                                                            start: 924,
                                                            end: 928,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fd209f60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRfGfphF/is_prime/src/main.sw",
                                        ),
                                        start: 413,
                                        end: 930,
                                        as_str(): "{\n    assert(check_prime(64) == false);\n    assert(check_prime(8) == false);\n    assert(check_prime(7) == true);\n    assert(check_prime(11) == true);\n    assert(check_prime(13) == true);\n    assert(check_prime(2) == true);\n    assert(check_prime(3) == true);\n    assert(check_prime(1) == false);\n    assert(check_prime(0) == false);\n\n    assert(check_prime(11) == check_prime(17));\n    assert(check_prime(12) == false);\n    assert(check_prime(18) == false);\n    assert(check_prime(12) == check_prime(18));\n\n    true\n}",
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
