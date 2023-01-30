Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe055f5d180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe055f5d180,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
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
                                        src (ptr): 0x00007fe055f5d180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
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
                                            src (ptr): 0x00007fe055f5d180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe055f5d180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
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
                                                src (ptr): 0x00007fe055f5d180,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 27,
                                                as_str(): "constants",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe055f5d180,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                ),
                                                start: 27,
                                                end: 29,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 29,
                                                    end: 38,
                                                    as_str(): "ZERO_B256",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe055f5d180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                        ),
                                        start: 38,
                                        end: 39,
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
                                            src (ptr): 0x00007fe055f5d180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                            ),
                                            start: 113,
                                            end: 115,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe055f5d180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                            ),
                                            start: 116,
                                            end: 120,
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
                                            src (ptr): 0x00007fe055f5d180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                            ),
                                            start: 120,
                                            end: 122,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe055f5d180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                    ),
                                                    start: 123,
                                                    end: 125,
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
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 126,
                                                                end: 130,
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
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 137,
                                                            end: 140,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 141,
                                                                end: 142,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 143,
                                                            end: 144,
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
                                                                        src (ptr): 0x00007fe055f5d180,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                        ),
                                                                        start: 145,
                                                                        end: 154,
                                                                        as_str(): "ZERO_B256",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 154,
                                                            end: 155,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Asm(
                                                    AsmBlock {
                                                        asm_token: AsmToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 160,
                                                                end: 163,
                                                                as_str(): "asm",
                                                            },
                                                        },
                                                        registers: Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        AsmRegisterDeclaration {
                                                                            register: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe055f5d180,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                    ),
                                                                                    start: 164,
                                                                                    end: 166,
                                                                                    as_str(): "r1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe055f5d180,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                            ),
                                                                                            start: 166,
                                                                                            end: 167,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe055f5d180,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                                        ),
                                                                                                        start: 168,
                                                                                                        end: 169,
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
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe055f5d180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                ),
                                                                                start: 169,
                                                                                end: 170,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    AsmRegisterDeclaration {
                                                                        register: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe055f5d180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                ),
                                                                                start: 171,
                                                                                end: 173,
                                                                                as_str(): "r2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        value_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe055f5d180,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                        ),
                                                                                        start: 173,
                                                                                        end: 174,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe055f5d180,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                                    ),
                                                                                                    start: 175,
                                                                                                    end: 184,
                                                                                                    as_str(): "ZERO_B256",
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
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 163,
                                                                end: 185,
                                                                as_str(): "(r1: a, r2: ZERO_B256)",
                                                            },
                                                        },
                                                        contents: Braces {
                                                            inner: AsmBlockContents {
                                                                instructions: [
                                                                    (
                                                                        Log {
                                                                            token: LogOpcode {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe055f5d180,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                    ),
                                                                                    start: 196,
                                                                                    end: 199,
                                                                                    as_str(): "log",
                                                                                },
                                                                            },
                                                                            reg_a: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe055f5d180,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                    ),
                                                                                    start: 200,
                                                                                    end: 202,
                                                                                    as_str(): "r1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            reg_b: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe055f5d180,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                    ),
                                                                                    start: 203,
                                                                                    end: 205,
                                                                                    as_str(): "r2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            reg_c: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe055f5d180,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                    ),
                                                                                    start: 206,
                                                                                    end: 210,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            reg_d: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe055f5d180,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                    ),
                                                                                    start: 211,
                                                                                    end: 215,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe055f5d180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                ),
                                                                                start: 215,
                                                                                end: 216,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_expr_opt: Some(
                                                                    AsmFinalExpr {
                                                                        register: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe055f5d180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                                ),
                                                                                start: 225,
                                                                                end: 229,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe055f5d180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 235,
                                                                as_str(): "{\n        log r1 r2 zero zero;\n        zero\n    }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 236,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 241,
                                                            end: 247,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe055f5d180,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                                            ),
                                                                            start: 248,
                                                                            end: 249,
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
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe055f5d180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                                            ),
                                                            start: 249,
                                                            end: 250,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe055f5d180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRvf33yY/retd_b256/src/main.sw",
                                        ),
                                        start: 131,
                                        end: 252,
                                        as_str(): "{\n    let a = ZERO_B256;\n    asm(r1: a, r2: ZERO_B256) {\n        log r1 r2 zero zero;\n        zero\n    };\n    return a;\n}",
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
