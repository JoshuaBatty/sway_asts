Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe073386450,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe073386450,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
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
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 19,
                                                as_str(): "*",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 19,
                                        end: 20,
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
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 21,
                                        end: 24,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 25,
                                            end: 28,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 28,
                                            end: 30,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 30,
                                                end: 36,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe073386450,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                ),
                                                start: 36,
                                                end: 38,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 38,
                                                    end: 44,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 44,
                                        end: 45,
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 47,
                                            end: 49,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 50,
                                            end: 58,
                                            as_str(): "sum_test",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 59,
                                                                        end: 60,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 60,
                                                                    end: 61,
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 61,
                                                                                end: 64,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 66,
                                                                        end: 67,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 67,
                                                                    end: 68,
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 68,
                                                                                end: 71,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 71,
                                                                end: 72,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 73,
                                                                    end: 74,
                                                                    as_str(): "c",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 75,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 78,
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 58,
                                            end: 79,
                                            as_str(): "(a:u64, b:u64, c:u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 80,
                                                    end: 82,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 83,
                                                                end: 86,
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
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 93,
                                                            end: 96,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 97,
                                                                end: 100,
                                                                as_str(): "sum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 102,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Add {
                                                        lhs: Add {
                                                            lhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 103,
                                                                                end: 104,
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
                                                            add_token: AddToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 106,
                                                                    as_str(): "+",
                                                                },
                                                            },
                                                            rhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 107,
                                                                                end: 108,
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
                                                        add_token: AddToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 109,
                                                                end: 110,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        rhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 111,
                                                                            end: 112,
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 113,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 121,
                                                                as_str(): "sum",
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
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 87,
                                        end: 123,
                                        as_str(): "{\n    let sum = a + b + c;\n    sum\n}",
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 125,
                                            end: 127,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 145,
                                            as_str(): "reassignment_test",
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
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 146,
                                                                    end: 150,
                                                                    as_str(): "cond",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 150,
                                                                end: 151,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 152,
                                                                            end: 156,
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 157,
                                            as_str(): "(cond: bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 158,
                                                    end: 160,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 161,
                                                                end: 164,
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
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 171,
                                                            end: 174,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 178,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 180,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 181,
                                                            end: 182,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 183,
                                                                    end: 184,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 184,
                                                            end: 185,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 190,
                                                                end: 192,
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 193,
                                                                                end: 197,
                                                                                as_str(): "cond",
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
                                                                statements: [
                                                                    Expr {
                                                                        expr: Reassignment {
                                                                            assignable: Var(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 208,
                                                                                        end: 209,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            reassignment_op: ReassignmentOp {
                                                                                variant: Equals,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 210,
                                                                                    end: 211,
                                                                                    as_str(): "=",
                                                                                },
                                                                            },
                                                                            expr: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 212,
                                                                                            end: 214,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                        parsed: 42,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 214,
                                                                                    end: 215,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 198,
                                                                end: 221,
                                                                as_str(): "{\n        b = 42;\n    }",
                                                            },
                                                        },
                                                        else_opt: Some(
                                                            (
                                                                ElseToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 222,
                                                                        end: 226,
                                                                        as_str(): "else",
                                                                    },
                                                                },
                                                                Break(
                                                                    Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [
                                                                                Expr {
                                                                                    expr: Reassignment {
                                                                                        assignable: Var(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 237,
                                                                                                    end: 238,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        reassignment_op: ReassignmentOp {
                                                                                            variant: Equals,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 239,
                                                                                                end: 240,
                                                                                                as_str(): "=",
                                                                                            },
                                                                                        },
                                                                                        expr: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 241,
                                                                                                        end: 242,
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 242,
                                                                                                end: 243,
                                                                                                as_str(): ";",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 227,
                                                                            end: 249,
                                                                            as_str(): "{\n        b = 5;\n    }",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 249,
                                                            end: 250,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 255,
                                                                end: 256,
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
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 165,
                                        end: 258,
                                        as_str(): "{\n    let mut b = 2;\n    if cond {\n        b = 42;\n    } else {\n        b = 5;\n    };\n    b\n}",
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 260,
                                            end: 262,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 263,
                                            end: 272,
                                            as_str(): "loop_test",
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
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 273,
                                                                    end: 283,
                                                                    as_str(): "trip_count",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 283,
                                                                end: 284,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 284,
                                                                            end: 287,
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 272,
                                            end: 288,
                                            as_str(): "(trip_count:u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 289,
                                                    end: 291,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 292,
                                                                end: 295,
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
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 302,
                                                            end: 305,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 306,
                                                                    end: 309,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 310,
                                                                end: 311,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 312,
                                                            end: 313,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 314,
                                                                    end: 315,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 315,
                                                            end: 316,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: While {
                                                    while_token: WhileToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 326,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 327,
                                                                            end: 328,
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
                                                        less_than_token: LessThanToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 329,
                                                                end: 330,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 331,
                                                                            end: 341,
                                                                            as_str(): "trip_count",
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
                                                                    expr: Reassignment {
                                                                        assignable: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 352,
                                                                                    end: 353,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        reassignment_op: ReassignmentOp {
                                                                            variant: Equals,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 354,
                                                                                end: 355,
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
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 356,
                                                                                                end: 357,
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
                                                                            add_token: AddToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 358,
                                                                                    end: 359,
                                                                                    as_str(): "+",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 360,
                                                                                            end: 361,
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 361,
                                                                                end: 362,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 342,
                                                            end: 368,
                                                            as_str(): "{\n        b = b + 1;\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Add {
                                                lhs: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 373,
                                                                    end: 374,
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
                                                add_token: AddToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe073386450,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                        ),
                                                        start: 375,
                                                        end: 376,
                                                        as_str(): "+",
                                                    },
                                                },
                                                rhs: Literal(
                                                    Int(
                                                        LitInt {
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 377,
                                                                end: 378,
                                                                as_str(): "1",
                                                            },
                                                            parsed: 1,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 296,
                                        end: 380,
                                        as_str(): "{\n    let mut b = 0;\n    while b < trip_count {\n        b = b + 1;\n    }\n    b + 1\n}",
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 382,
                                            end: 384,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 385,
                                            end: 389,
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
                                            src (ptr): 0x00007fe073386450,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                            ),
                                            start: 389,
                                            end: 391,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe073386450,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                    ),
                                                    start: 392,
                                                    end: 394,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 395,
                                                                end: 398,
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
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 405,
                                                            end: 408,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 409,
                                                                end: 413,
                                                                as_str(): "four",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 414,
                                                            end: 415,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 416,
                                                                    end: 417,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 417,
                                                            end: 418,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 423,
                                                            end: 426,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 427,
                                                                end: 432,
                                                                as_str(): "three",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 433,
                                                            end: 434,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe073386450,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                    ),
                                                                    start: 435,
                                                                    end: 436,
                                                                    as_str(): "3",
                                                                },
                                                                parsed: 3,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 436,
                                                            end: 437,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 442,
                                                            end: 445,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 446,
                                                                end: 449,
                                                                as_str(): "sum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 450,
                                                            end: 451,
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 452,
                                                                            end: 456,
                                                                            as_str(): "four",
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 457,
                                                                end: 458,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        rhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 459,
                                                                            end: 464,
                                                                            as_str(): "three",
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
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 465,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 470,
                                                                        end: 476,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 477,
                                                                                        end: 480,
                                                                                        as_str(): "sum",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 481,
                                                                            end: 483,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 484,
                                                                                    end: 485,
                                                                                    as_str(): "7",
                                                                                },
                                                                                parsed: 7,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 476,
                                                            end: 486,
                                                            as_str(): "(sum == 7)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 486,
                                                            end: 487,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 493,
                                                                        end: 499,
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
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 500,
                                                                                end: 504,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 499,
                                                            end: 505,
                                                            as_str(): "(true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 505,
                                                            end: 506,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 511,
                                                                        end: 517,
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
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 518,
                                                                                            end: 527,
                                                                                            as_str(): "loop_test",
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
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 528,
                                                                                                    end: 530,
                                                                                                    as_str(): "10",
                                                                                                },
                                                                                                parsed: 10,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 527,
                                                                                end: 531,
                                                                                as_str(): "(10)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 532,
                                                                            end: 534,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 535,
                                                                                    end: 537,
                                                                                    as_str(): "11",
                                                                                },
                                                                                parsed: 11,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 517,
                                                            end: 538,
                                                            as_str(): "(loop_test(10) == 11)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 538,
                                                            end: 539,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 544,
                                                                        end: 550,
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
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 551,
                                                                                            end: 568,
                                                                                            as_str(): "reassignment_test",
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
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 569,
                                                                                                    end: 574,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 568,
                                                                                end: 575,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 576,
                                                                            end: 578,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 579,
                                                                                    end: 580,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 550,
                                                            end: 581,
                                                            as_str(): "(reassignment_test(false) == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 581,
                                                            end: 582,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 587,
                                                                        end: 593,
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
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 594,
                                                                                            end: 602,
                                                                                            as_str(): "sum_test",
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
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 603,
                                                                                                        end: 604,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                    parsed: 1,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 604,
                                                                                                end: 605,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 606,
                                                                                                        end: 607,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                    parsed: 2,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 607,
                                                                                                end: 608,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 609,
                                                                                                    end: 610,
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 602,
                                                                                end: 611,
                                                                                as_str(): "(1, 2, 3)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 612,
                                                                            end: 614,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 615,
                                                                                    end: 616,
                                                                                    as_str(): "6",
                                                                                },
                                                                                parsed: 6,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 593,
                                                            end: 617,
                                                            as_str(): "(sum_test(1, 2, 3) == 6)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 618,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 623,
                                                                        end: 629,
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
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 630,
                                                                                            end: 638,
                                                                                            as_str(): "sum_test",
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
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 639,
                                                                                                        end: 641,
                                                                                                        as_str(): "30",
                                                                                                    },
                                                                                                    parsed: 30,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 641,
                                                                                                end: 642,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 643,
                                                                                                        end: 645,
                                                                                                        as_str(): "20",
                                                                                                    },
                                                                                                    parsed: 20,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 645,
                                                                                                end: 646,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 647,
                                                                                                    end: 649,
                                                                                                    as_str(): "10",
                                                                                                },
                                                                                                parsed: 10,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 638,
                                                                                end: 650,
                                                                                as_str(): "(30, 20, 10)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 651,
                                                                            end: 653,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 654,
                                                                                    end: 656,
                                                                                    as_str(): "60",
                                                                                },
                                                                                parsed: 60,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 629,
                                                            end: 657,
                                                            as_str(): "(sum_test(30, 20, 10) == 60)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 657,
                                                            end: 658,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 663,
                                                                        end: 669,
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
                                                                                            src (ptr): 0x00007fe073386450,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                            ),
                                                                                            start: 670,
                                                                                            end: 678,
                                                                                            as_str(): "sum_test",
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
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 679,
                                                                                                        end: 680,
                                                                                                        as_str(): "3",
                                                                                                    },
                                                                                                    parsed: 3,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 680,
                                                                                                end: 681,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe073386450,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 682,
                                                                                                        end: 683,
                                                                                                        as_str(): "2",
                                                                                                    },
                                                                                                    parsed: 2,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe073386450,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                ),
                                                                                                start: 683,
                                                                                                end: 684,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe073386450,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 685,
                                                                                                    end: 686,
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
                                                                                src (ptr): 0x00007fe073386450,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                ),
                                                                                start: 678,
                                                                                end: 687,
                                                                                as_str(): "(3, 2, 1)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 688,
                                                                            end: 690,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 691,
                                                                                    end: 692,
                                                                                    as_str(): "6",
                                                                                },
                                                                                parsed: 6,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 669,
                                                            end: 693,
                                                            as_str(): "(sum_test(3, 2, 1) == 6)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 693,
                                                            end: 694,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 700,
                                                            end: 703,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 704,
                                                                end: 707,
                                                                as_str(): "res",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 708,
                                                            end: 709,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FuncApp {
                                                        func: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 710,
                                                                            end: 727,
                                                                            as_str(): "reassignment_test",
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
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 728,
                                                                                    end: 732,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 727,
                                                                end: 733,
                                                                as_str(): "(true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 733,
                                                            end: 734,
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
                                                                        src (ptr): 0x00007fe073386450,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                        ),
                                                                        start: 739,
                                                                        end: 745,
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
                                                                                        src (ptr): 0x00007fe073386450,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                        ),
                                                                                        start: 746,
                                                                                        end: 749,
                                                                                        as_str(): "res",
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
                                                                            src (ptr): 0x00007fe073386450,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                            ),
                                                                            start: 750,
                                                                            end: 752,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe073386450,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                                    ),
                                                                                    start: 753,
                                                                                    end: 755,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 745,
                                                            end: 756,
                                                            as_str(): "(res == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe073386450,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                            ),
                                                            start: 756,
                                                            end: 757,
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
                                                                src (ptr): 0x00007fe073386450,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                                                ),
                                                                start: 762,
                                                                end: 765,
                                                                as_str(): "res",
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
                                        src (ptr): 0x00007fe073386450,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAfp6LR/new_allocator_test/src/main.sw",
                                        ),
                                        start: 399,
                                        end: 767,
                                        as_str(): "{\n    let four = 4;\n    let three = 3;\n    let sum = four + three;\n    assert(sum == 7);\n\n    assert(true);\n    assert(loop_test(10) == 11);\n    assert(reassignment_test(false) == 5);\n    assert(sum_test(1, 2, 3) == 6);\n    assert(sum_test(30, 20, 10) == 60);\n    assert(sum_test(3, 2, 1) == 6);\n\n    let res = reassignment_test(true);\n    assert(res == 42);\n    res\n}",
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
