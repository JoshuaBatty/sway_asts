Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0503ea450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0503ea450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
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
                                                src (ptr): 0x00007fe0503ea450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0503ea450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                ),
                                                start: 24,
                                                end: 26,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 32,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 32,
                                        end: 33,
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
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 38,
                                            end: 42,
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
                                            src (ptr): 0x00007fe0503ea450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                            ),
                                            start: 42,
                                            end: 44,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0503ea450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 47,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 48,
                                                                end: 51,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 58,
                                                            end: 61,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 62,
                                                                    end: 65,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 66,
                                                                end: 67,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 68,
                                                            end: 69,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 70,
                                                                    end: 71,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 71,
                                                            end: 72,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Var(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 82,
                                                                end: 83,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: AddEquals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 84,
                                                            end: 86,
                                                            as_str(): "+=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 87,
                                                                    end: 89,
                                                                    as_str(): "99",
                                                                },
                                                                parsed: 99,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 89,
                                                            end: 90,
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
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 95,
                                                                        end: 101,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 102,
                                                                                        end: 103,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 104,
                                                                            end: 106,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 107,
                                                                                    end: 109,
                                                                                    as_str(): "99",
                                                                                },
                                                                                parsed: 99,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 110,
                                                            as_str(): "(a == 99)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 110,
                                                            end: 111,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 117,
                                                                end: 118,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: SubEquals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 121,
                                                            as_str(): "-=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 123,
                                                                    as_str(): "5",
                                                                },
                                                                parsed: 5,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 123,
                                                            end: 124,
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
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 129,
                                                                        end: 135,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 136,
                                                                                        end: 137,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 138,
                                                                            end: 140,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 141,
                                                                                    end: 143,
                                                                                    as_str(): "94",
                                                                                },
                                                                                parsed: 94,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 144,
                                                            as_str(): "(a == 94)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 144,
                                                            end: 145,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 151,
                                                                end: 152,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: MulEquals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 153,
                                                            end: 155,
                                                            as_str(): "*=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 156,
                                                                    end: 157,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 158,
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
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 163,
                                                                        end: 169,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 170,
                                                                                        end: 171,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 174,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 175,
                                                                                    end: 178,
                                                                                    as_str(): "188",
                                                                                },
                                                                                parsed: 188,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 179,
                                                            as_str(): "(a == 188)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 179,
                                                            end: 180,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 187,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: DivEquals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 188,
                                                            end: 190,
                                                            as_str(): "/=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 191,
                                                                    end: 193,
                                                                    as_str(): "47",
                                                                },
                                                                parsed: 47,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 193,
                                                            end: 194,
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
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 199,
                                                                        end: 205,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 206,
                                                                                        end: 207,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 208,
                                                                            end: 210,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 211,
                                                                                    end: 212,
                                                                                    as_str(): "4",
                                                                                },
                                                                                parsed: 4,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 205,
                                                            end: 213,
                                                            as_str(): "(a == 4)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 213,
                                                            end: 214,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 220,
                                                                end: 221,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 222,
                                                            end: 223,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 224,
                                                                    end: 227,
                                                                    as_str(): "999",
                                                                },
                                                                parsed: 999,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 227,
                                                            end: 228,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 234,
                                                                end: 235,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: ShrEquals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 236,
                                                            end: 239,
                                                            as_str(): ">>=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 240,
                                                                    end: 241,
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
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 241,
                                                            end: 242,
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
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 247,
                                                                        end: 253,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 254,
                                                                                        end: 255,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 256,
                                                                            end: 258,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 259,
                                                                                    end: 262,
                                                                                    as_str(): "499",
                                                                                },
                                                                                parsed: 499,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 263,
                                                            as_str(): "(a == 499)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 263,
                                                            end: 264,
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
                                                                src (ptr): 0x00007fe0503ea450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                ),
                                                                start: 270,
                                                                end: 271,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: ShlEquals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 272,
                                                            end: 275,
                                                            as_str(): "<<=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0503ea450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                    ),
                                                                    start: 276,
                                                                    end: 277,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 277,
                                                            end: 278,
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
                                                                        src (ptr): 0x00007fe0503ea450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                        ),
                                                                        start: 283,
                                                                        end: 289,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0503ea450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                        ),
                                                                                        start: 290,
                                                                                        end: 291,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0503ea450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 294,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0503ea450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                                                    ),
                                                                                    start: 295,
                                                                                    end: 299,
                                                                                    as_str(): "1996",
                                                                                },
                                                                                parsed: 1996,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 289,
                                                            end: 300,
                                                            as_str(): "(a == 1996)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 300,
                                                            end: 301,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0503ea450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                                            ),
                                                            start: 307,
                                                            end: 308,
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
                                        src (ptr): 0x00007fe0503ea450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRu4PJqV/reassignment_operators/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 310,
                                        as_str(): "{\n    let mut a = 0;\n    \n    a += 99;\n    assert(a == 99);\n\n    a -= 5;\n    assert(a == 94);\n\n    a *= 2;\n    assert(a == 188);\n\n    a /= 47;\n    assert(a == 4);\n\n    a = 999;\n\n    a >>= 1;\n    assert(a == 499);\n\n    a <<= 2;\n    assert(a == 1996);\n\n    1\n}",
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
