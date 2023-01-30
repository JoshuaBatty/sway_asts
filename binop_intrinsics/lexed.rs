Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb14d3784f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb14d3784f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                src (ptr): 0x00007fb14d3784f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb14d3784f0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                    src (ptr): 0x00007fb14d3784f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                            src (ptr): 0x00007fb14d3784f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                    src (ptr): 0x00007fb14d3784f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 60,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 62,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 62,
                                                                    end: 63,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 64,
                                                                                end: 66,
                                                                                as_str(): "u8",
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 68,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 69,
                                                                    end: 70,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 70,
                                                            end: 71,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 74,
                                                            end: 77,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 78,
                                                                end: 79,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 80,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 81,
                                                                                end: 83,
                                                                                as_str(): "u8",
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 84,
                                                            end: 85,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 86,
                                                                    end: 88,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 88,
                                                            end: 89,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 92,
                                                                        end: 98,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 99,
                                                                                            end: 104,
                                                                                            as_str(): "__add",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 105,
                                                                                                            end: 106,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 106,
                                                                                                end: 107,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 108,
                                                                                                        end: 109,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 104,
                                                                                end: 110,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 111,
                                                                            end: 113,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 114,
                                                                                    end: 116,
                                                                                    as_str(): "24",
                                                                                },
                                                                                parsed: 24,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 117,
                                                            as_str(): "(__add(a, b) == 24)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 117,
                                                            end: 118,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 121,
                                                                        end: 127,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 128,
                                                                                            end: 133,
                                                                                            as_str(): "__sub",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 134,
                                                                                                            end: 135,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 135,
                                                                                                end: 136,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 137,
                                                                                                        end: 138,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 133,
                                                                                end: 139,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 142,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 143,
                                                                                    end: 145,
                                                                                    as_str(): "20",
                                                                                },
                                                                                parsed: 20,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 127,
                                                            end: 146,
                                                            as_str(): "(__sub(b, a) == 20)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 147,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 150,
                                                                        end: 156,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 157,
                                                                                            end: 162,
                                                                                            as_str(): "__mul",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 163,
                                                                                                            end: 164,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 164,
                                                                                                end: 165,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 166,
                                                                                                        end: 167,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 162,
                                                                                end: 168,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 169,
                                                                            end: 171,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 174,
                                                                                    as_str(): "44",
                                                                                },
                                                                                parsed: 44,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 156,
                                                            end: 175,
                                                            as_str(): "(__mul(a, b) == 44)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 176,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 179,
                                                                        end: 185,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 186,
                                                                                            end: 191,
                                                                                            as_str(): "__div",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 192,
                                                                                                            end: 193,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 193,
                                                                                                end: 194,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 195,
                                                                                                        end: 196,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 191,
                                                                                end: 197,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 198,
                                                                            end: 200,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 201,
                                                                                    end: 203,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 204,
                                                            as_str(): "(__div(b, a) == 11)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 204,
                                                            end: 205,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 209,
                                                            end: 212,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 213,
                                                                end: 214,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 214,
                                                                    end: 215,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 216,
                                                                                end: 219,
                                                                                as_str(): "u16",
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 220,
                                                            end: 221,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 222,
                                                                    end: 224,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 224,
                                                            end: 225,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 228,
                                                            end: 231,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 232,
                                                                end: 233,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 233,
                                                                    end: 234,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 235,
                                                                                end: 238,
                                                                                as_str(): "u16",
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 241,
                                                                    end: 243,
                                                                    as_str(): "44",
                                                                },
                                                                parsed: 44,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 243,
                                                            end: 244,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 254,
                                                                                            end: 259,
                                                                                            as_str(): "__add",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 260,
                                                                                                            end: 261,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 261,
                                                                                                end: 262,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 263,
                                                                                                        end: 264,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 259,
                                                                                end: 265,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 266,
                                                                            end: 268,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 269,
                                                                                    end: 271,
                                                                                    as_str(): "66",
                                                                                },
                                                                                parsed: 66,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 272,
                                                            as_str(): "(__add(a, b) == 66)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 272,
                                                            end: 273,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 276,
                                                                        end: 282,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 283,
                                                                                            end: 288,
                                                                                            as_str(): "__sub",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 289,
                                                                                                            end: 290,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 290,
                                                                                                end: 291,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 292,
                                                                                                        end: 293,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 288,
                                                                                end: 294,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 295,
                                                                            end: 297,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 298,
                                                                                    end: 300,
                                                                                    as_str(): "22",
                                                                                },
                                                                                parsed: 22,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 301,
                                                            as_str(): "(__sub(b, a) == 22)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 302,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 305,
                                                                        end: 311,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 312,
                                                                                            end: 317,
                                                                                            as_str(): "__mul",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 318,
                                                                                                            end: 319,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 319,
                                                                                                end: 320,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 321,
                                                                                                        end: 322,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 317,
                                                                                end: 323,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 324,
                                                                            end: 326,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 327,
                                                                                    end: 330,
                                                                                    as_str(): "968",
                                                                                },
                                                                                parsed: 968,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 311,
                                                            end: 331,
                                                            as_str(): "(__mul(a, b) == 968)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 331,
                                                            end: 332,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 335,
                                                                        end: 341,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 342,
                                                                                            end: 347,
                                                                                            as_str(): "__div",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 348,
                                                                                                            end: 349,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 349,
                                                                                                end: 350,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 351,
                                                                                                        end: 352,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 347,
                                                                                end: 353,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 354,
                                                                            end: 356,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 357,
                                                                                    end: 358,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 341,
                                                            end: 359,
                                                            as_str(): "(__div(b, a) == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 359,
                                                            end: 360,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 364,
                                                            end: 367,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 368,
                                                                end: 369,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 369,
                                                                    end: 370,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 371,
                                                                                end: 374,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 375,
                                                            end: 376,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 377,
                                                                    end: 379,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 379,
                                                            end: 380,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 383,
                                                            end: 386,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 387,
                                                                end: 388,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 388,
                                                                    end: 389,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 390,
                                                                                end: 393,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 394,
                                                            end: 395,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 396,
                                                                    end: 398,
                                                                    as_str(): "44",
                                                                },
                                                                parsed: 44,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 398,
                                                            end: 399,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 402,
                                                                        end: 408,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 409,
                                                                                            end: 414,
                                                                                            as_str(): "__add",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 415,
                                                                                                            end: 416,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 416,
                                                                                                end: 417,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 418,
                                                                                                        end: 419,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 414,
                                                                                end: 420,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 421,
                                                                            end: 423,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 424,
                                                                                    end: 426,
                                                                                    as_str(): "66",
                                                                                },
                                                                                parsed: 66,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 408,
                                                            end: 427,
                                                            as_str(): "(__add(a, b) == 66)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 427,
                                                            end: 428,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 431,
                                                                        end: 437,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 438,
                                                                                            end: 443,
                                                                                            as_str(): "__sub",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 444,
                                                                                                            end: 445,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 445,
                                                                                                end: 446,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 447,
                                                                                                        end: 448,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 443,
                                                                                end: 449,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 450,
                                                                            end: 452,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 453,
                                                                                    end: 455,
                                                                                    as_str(): "22",
                                                                                },
                                                                                parsed: 22,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 437,
                                                            end: 456,
                                                            as_str(): "(__sub(b, a) == 22)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 456,
                                                            end: 457,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 460,
                                                                        end: 466,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 467,
                                                                                            end: 472,
                                                                                            as_str(): "__mul",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 473,
                                                                                                            end: 474,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 474,
                                                                                                end: 475,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 476,
                                                                                                        end: 477,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 472,
                                                                                end: 478,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 479,
                                                                            end: 481,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 482,
                                                                                    end: 485,
                                                                                    as_str(): "968",
                                                                                },
                                                                                parsed: 968,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 466,
                                                            end: 486,
                                                            as_str(): "(__mul(a, b) == 968)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 490,
                                                                        end: 496,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 497,
                                                                                            end: 502,
                                                                                            as_str(): "__div",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 503,
                                                                                                            end: 504,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 504,
                                                                                                end: 505,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 506,
                                                                                                        end: 507,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 502,
                                                                                end: 508,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 509,
                                                                            end: 511,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 512,
                                                                                    end: 513,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 496,
                                                            end: 514,
                                                            as_str(): "(__div(b, a) == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 514,
                                                            end: 515,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 519,
                                                            end: 522,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 523,
                                                                end: 524,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 524,
                                                                    end: 525,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 526,
                                                                                end: 529,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 530,
                                                            end: 531,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 532,
                                                                    end: 534,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 534,
                                                            end: 535,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 538,
                                                            end: 541,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14d3784f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                ),
                                                                start: 542,
                                                                end: 543,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 543,
                                                                    end: 544,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 545,
                                                                                end: 548,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 549,
                                                            end: 550,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                    ),
                                                                    start: 551,
                                                                    end: 553,
                                                                    as_str(): "44",
                                                                },
                                                                parsed: 44,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 553,
                                                            end: 554,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 557,
                                                                        end: 563,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 564,
                                                                                            end: 569,
                                                                                            as_str(): "__add",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 570,
                                                                                                            end: 571,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 571,
                                                                                                end: 572,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 573,
                                                                                                        end: 574,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 569,
                                                                                end: 575,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
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
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 579,
                                                                                    end: 581,
                                                                                    as_str(): "66",
                                                                                },
                                                                                parsed: 66,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 563,
                                                            end: 582,
                                                            as_str(): "(__add(a, b) == 66)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 582,
                                                            end: 583,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 586,
                                                                        end: 592,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 593,
                                                                                            end: 598,
                                                                                            as_str(): "__sub",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 599,
                                                                                                            end: 600,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 600,
                                                                                                end: 601,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 602,
                                                                                                        end: 603,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 598,
                                                                                end: 604,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 605,
                                                                            end: 607,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 608,
                                                                                    end: 610,
                                                                                    as_str(): "22",
                                                                                },
                                                                                parsed: 22,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 592,
                                                            end: 611,
                                                            as_str(): "(__sub(b, a) == 22)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 612,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 615,
                                                                        end: 621,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 622,
                                                                                            end: 627,
                                                                                            as_str(): "__mul",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 628,
                                                                                                            end: 629,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 629,
                                                                                                end: 630,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 631,
                                                                                                        end: 632,
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
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 627,
                                                                                end: 633,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 634,
                                                                            end: 636,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 637,
                                                                                    end: 640,
                                                                                    as_str(): "968",
                                                                                },
                                                                                parsed: 968,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 621,
                                                            end: 641,
                                                            as_str(): "(__mul(a, b) == 968)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 641,
                                                            end: 642,
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
                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                        ),
                                                                        start: 645,
                                                                        end: 651,
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
                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                            ),
                                                                                            start: 652,
                                                                                            end: 657,
                                                                                            as_str(): "__div",
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 658,
                                                                                                            end: 659,
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
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                ),
                                                                                                start: 659,
                                                                                                end: 660,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14d3784f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 661,
                                                                                                        end: 662,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14d3784f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                ),
                                                                                start: 657,
                                                                                end: 663,
                                                                                as_str(): "(b, a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14d3784f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                            ),
                                                                            start: 664,
                                                                            end: 666,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14d3784f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                                                    ),
                                                                                    start: 667,
                                                                                    end: 668,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 651,
                                                            end: 669,
                                                            as_str(): "(__div(b, a) == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 669,
                                                            end: 670,
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
                                                            src (ptr): 0x00007fb14d3784f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                                            ),
                                                            start: 675,
                                                            end: 676,
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
                                        src (ptr): 0x00007fb14d3784f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Zxa6O/binop_intrinsics/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 678,
                                        as_str(): "{\n\n  let a: u8 = 2;\n  let b: u8 = 22;\n  assert(__add(a, b) == 24);\n  assert(__sub(b, a) == 20);\n  assert(__mul(a, b) == 44);\n  assert(__div(b, a) == 11);\n\n  let a: u16 = 22;\n  let b: u16 = 44;\n  assert(__add(a, b) == 66);\n  assert(__sub(b, a) == 22);\n  assert(__mul(a, b) == 968);\n  assert(__div(b, a) == 2);\n\n  let a: u32 = 22;\n  let b: u32 = 44;\n  assert(__add(a, b) == 66);\n  assert(__sub(b, a) == 22);\n  assert(__mul(a, b) == 968);\n  assert(__div(b, a) == 2);\n\n  let a: u64 = 22;\n  let b: u64 = 44;\n  assert(__add(a, b) == 66);\n  assert(__sub(b, a) == 22);\n  assert(__mul(a, b) == 968);\n  assert(__div(b, a) == 2);\n\n\n  2\n}",
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
