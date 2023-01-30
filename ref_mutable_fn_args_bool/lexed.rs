Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06da36640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06da36640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                                                        src (ptr): 0x00007fe06da36640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                                                        src (ptr): 0x00007fe06da36640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                                                    src (ptr): 0x00007fe06da36640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                                                src (ptr): 0x00007fe06da36640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
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
                                                                            src (ptr): 0x00007fe06da36640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                            ),
                                                                            start: 31,
                                                                            end: 35,
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
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 19,
                                            end: 36,
                                            as_str(): "(ref mut b: bool)",
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
                                                                src (ptr): 0x00007fe06da36640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                ),
                                                                start: 43,
                                                                end: 44,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 45,
                                                            end: 46,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06da36640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                    ),
                                                                    start: 47,
                                                                    end: 51,
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
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 51,
                                                            end: 52,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06da36640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 54,
                                        as_str(): "{\n    b = true;\n}",
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
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 56,
                                            end: 58,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 63,
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
                                            src (ptr): 0x00007fe06da36640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                            ),
                                            start: 63,
                                            end: 65,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe06da36640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                    ),
                                                    start: 66,
                                                    end: 68,
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
                                                                src (ptr): 0x00007fe06da36640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                ),
                                                                start: 69,
                                                                end: 73,
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
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 80,
                                                            end: 83,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06da36640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                    ),
                                                                    start: 84,
                                                                    end: 87,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe06da36640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                ),
                                                                start: 88,
                                                                end: 89,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 91,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06da36640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                    ),
                                                                    start: 92,
                                                                    end: 97,
                                                                    as_str(): "false",
                                                                },
                                                                kind: False,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 97,
                                                            end: 98,
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
                                                                        src (ptr): 0x00007fe06da36640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                        ),
                                                                        start: 103,
                                                                        end: 110,
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
                                                                                    src (ptr): 0x00007fe06da36640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                                    ),
                                                                                    start: 111,
                                                                                    end: 112,
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
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 110,
                                                            end: 113,
                                                            as_str(): "(b)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06da36640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 114,
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
                                                                src (ptr): 0x00007fe06da36640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                                                ),
                                                                start: 119,
                                                                end: 120,
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
                                        src (ptr): 0x00007fe06da36640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHSeZOg/ref_mutable_fn_args_bool/src/main.sw",
                                        ),
                                        start: 74,
                                        end: 122,
                                        as_str(): "{\n    let mut b = false;\n    mut_arg(b);\n    b\n}",
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
