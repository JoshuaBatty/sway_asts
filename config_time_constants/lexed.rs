Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb125da2240,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb125da2240,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
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
                                            src (ptr): 0x00007fb125da2240,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb125da2240,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
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
                                            src (ptr): 0x00007fb125da2240,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
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
                                                    src (ptr): 0x00007fb125da2240,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
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
                                                                src (ptr): 0x00007fb125da2240,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
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
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb125da2240,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                            ),
                                                            start: 32,
                                                            end: 35,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb125da2240,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                ),
                                                                start: 36,
                                                                end: 40,
                                                                as_str(): "addr",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb125da2240,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                            ),
                                                            start: 41,
                                                            end: 42,
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
                                                                        src (ptr): 0x00007fb125da2240,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                        ),
                                                                        start: 43,
                                                                        end: 61,
                                                                        as_str(): "some_contract_addr",
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
                                                            src (ptr): 0x00007fb125da2240,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                            ),
                                                            start: 61,
                                                            end: 62,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb125da2240,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                            ),
                                                            start: 68,
                                                            end: 74,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        If(
                                                            IfExpr {
                                                                if_token: IfToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb125da2240,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                        ),
                                                                        start: 75,
                                                                        end: 77,
                                                                        as_str(): "if",
                                                                    },
                                                                },
                                                                condition: Expr(
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb125da2240,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                                        ),
                                                                                        start: 78,
                                                                                        end: 87,
                                                                                        as_str(): "true_bool",
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
                                                                then_block: Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb125da2240,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 90,
                                                                                                end: 98,
                                                                                                as_str(): "some_num",
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
                                                                        src (ptr): 0x00007fb125da2240,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                        ),
                                                                        start: 88,
                                                                        end: 100,
                                                                        as_str(): "{ some_num }",
                                                                    },
                                                                },
                                                                else_opt: Some(
                                                                    (
                                                                        ElseToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb125da2240,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                                ),
                                                                                start: 101,
                                                                                end: 105,
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
                                                                                                        src (ptr): 0x00007fb125da2240,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                                                        ),
                                                                                                        start: 108,
                                                                                                        end: 109,
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
                                                                                    src (ptr): 0x00007fb125da2240,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 106,
                                                                                    end: 111,
                                                                                    as_str(): "{ 0 }",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb125da2240,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 113,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb125da2240,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmcNPLS/config_time_constants/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 115,
                                        as_str(): "{\n    let addr = some_contract_addr;\n\n    return if true_bool { some_num } else { 0 } ;\n}",
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
