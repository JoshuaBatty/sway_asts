Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe05beff8e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe05beff8e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
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
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 235,
                                            end: 237,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 238,
                                            end: 241,
                                            as_str(): "foo",
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
                                                                        src (ptr): 0x00007fe05beff8e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                        ),
                                                                        start: 242,
                                                                        end: 245,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05beff8e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                        ),
                                                                        start: 246,
                                                                        end: 249,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                    ),
                                                                    start: 250,
                                                                    end: 251,
                                                                    as_str(): "x",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe05beff8e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                ),
                                                                start: 251,
                                                                end: 252,
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
                                                                            src (ptr): 0x00007fe05beff8e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                            ),
                                                                            start: 253,
                                                                            end: 256,
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
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 241,
                                            end: 257,
                                            as_str(): "(ref mut x: u64)",
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
                                                                src (ptr): 0x00007fe05beff8e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                ),
                                                                start: 264,
                                                                end: 265,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: AddEquals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 268,
                                                            as_str(): "+=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                    ),
                                                                    start: 269,
                                                                    end: 270,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 271,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05beff8e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                        ),
                                        start: 258,
                                        end: 273,
                                        as_str(): "{\n    x += 1;\n}",
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
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 275,
                                            end: 277,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 278,
                                            end: 282,
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
                                            src (ptr): 0x00007fe05beff8e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                            ),
                                            start: 282,
                                            end: 284,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05beff8e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                    ),
                                                    start: 285,
                                                    end: 287,
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
                                                                src (ptr): 0x00007fe05beff8e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                ),
                                                                start: 288,
                                                                end: 291,
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
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 301,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                    ),
                                                                    start: 302,
                                                                    end: 305,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05beff8e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                ),
                                                                start: 306,
                                                                end: 307,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 308,
                                                            end: 309,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                    ),
                                                                    start: 310,
                                                                    end: 311,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 311,
                                                            end: 312,
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
                                                                        src (ptr): 0x00007fe05beff8e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                        ),
                                                                        start: 317,
                                                                        end: 320,
                                                                        as_str(): "foo",
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
                                                                                    src (ptr): 0x00007fe05beff8e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                                    ),
                                                                                    start: 321,
                                                                                    end: 322,
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 320,
                                                            end: 323,
                                                            as_str(): "(x)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05beff8e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 324,
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
                                                                src (ptr): 0x00007fe05beff8e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                                                ),
                                                                start: 329,
                                                                end: 330,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05beff8e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRnydIk8/ref_mutable_fn_args_call/src/main.sw",
                                        ),
                                        start: 292,
                                        end: 332,
                                        as_str(): "{\n    let mut x = 1;\n    foo(x);\n    x\n}",
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
