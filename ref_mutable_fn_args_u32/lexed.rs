Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06de68e50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06de68e50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
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
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 19,
                                            as_str(): "mut_arg",
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
                                                            reference: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06de68e50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                        ),
                                                                        start: 20,
                                                                        end: 23,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06de68e50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                        ),
                                                                        start: 24,
                                                                        end: 27,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06de68e50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                    ),
                                                                    start: 28,
                                                                    end: 29,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06de68e50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                ),
                                                                start: 29,
                                                                end: 30,
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
                                                                            src (ptr): 0x00007fe06de68e50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                            ),
                                                                            start: 31,
                                                                            end: 34,
                                                                            as_str(): "u32",
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
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 35,
                                            as_str(): "(ref mut b: u32)",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Var(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06de68e50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                ),
                                                                start: 42,
                                                                end: 43,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 44,
                                                            end: 45,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06de68e50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                    ),
                                                                    start: 46,
                                                                    end: 48,
                                                                    as_str(): "20",
                                                                },
                                                                parsed: 20,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 48,
                                                            end: 49,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06de68e50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                        ),
                                        start: 36,
                                        end: 51,
                                        as_str(): "{\n    b = 20;\n}",
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
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 53,
                                            end: 55,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 56,
                                            end: 60,
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
                                            src (ptr): 0x00007fe06de68e50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                            ),
                                            start: 60,
                                            end: 62,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06de68e50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                    ),
                                                    start: 63,
                                                    end: 65,
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
                                                                src (ptr): 0x00007fe06de68e50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 69,
                                                                as_str(): "u32",
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
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 76,
                                                            end: 79,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06de68e50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                    ),
                                                                    start: 80,
                                                                    end: 83,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06de68e50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                ),
                                                                start: 84,
                                                                end: 85,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 87,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06de68e50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 89,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: Some(
                                                                    (
                                                                        U32,
                                                                        Span {
                                                                            src (ptr): 0x00007fe06de68e50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                            ),
                                                                            start: 89,
                                                                            end: 92,
                                                                            as_str(): "u32",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 93,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06de68e50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                        ),
                                                                        start: 98,
                                                                        end: 105,
                                                                        as_str(): "mut_arg",
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
                                                                                    src (ptr): 0x00007fe06de68e50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
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
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 105,
                                                            end: 108,
                                                            as_str(): "(b)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06de68e50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 109,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06de68e50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                                                ),
                                                                start: 114,
                                                                end: 115,
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
                                        src (ptr): 0x00007fe06de68e50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIREF9KFT/ref_mutable_fn_args_u32/src/main.sw",
                                        ),
                                        start: 70,
                                        end: 117,
                                        as_str(): "{\n    let mut b = 0u32;\n    mut_arg(b);\n    b\n}",
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
