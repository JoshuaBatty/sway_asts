Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0e931aa40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0e931aa40,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                src (ptr): 0x00007fb0e931aa40,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb0e931aa40,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                            src (ptr): 0x00007fb0e931aa40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                    src (ptr): 0x00007fb0e931aa40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 57,
                                                                        end: 63,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 64,
                                                                                            end: 68,
                                                                                            as_str(): "__eq",
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
                                                                                            Bool(
                                                                                                LitBool {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 69,
                                                                                                        end: 73,
                                                                                                        as_str(): "true",
                                                                                                    },
                                                                                                    kind: True,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 73,
                                                                                                end: 74,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 75,
                                                                                                    end: 79,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 68,
                                                                                end: 80,
                                                                                as_str(): "(true, true)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 81,
                                                                            end: 83,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 85,
                                                                                                end: 89,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                double_eq_token: DoubleEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 90,
                                                                                        end: 92,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                rhs: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 93,
                                                                                                end: 97,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 84,
                                                                                end: 98,
                                                                                as_str(): "(true == true)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 63,
                                                            end: 99,
                                                            as_str(): "(__eq(true, true) == (true == true))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 99,
                                                            end: 100,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 103,
                                                                        end: 109,
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
                                                                NotEqual {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 110,
                                                                                            end: 114,
                                                                                            as_str(): "__eq",
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
                                                                                            Bool(
                                                                                                LitBool {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 115,
                                                                                                        end: 119,
                                                                                                        as_str(): "true",
                                                                                                    },
                                                                                                    kind: True,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 119,
                                                                                                end: 120,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 121,
                                                                                                    end: 126,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 114,
                                                                                end: 127,
                                                                                as_str(): "(true, false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 128,
                                                                            end: 130,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: NotEqual {
                                                                                lhs: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 132,
                                                                                                end: 136,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                bang_eq_token: BangEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 137,
                                                                                        end: 139,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                },
                                                                                rhs: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 140,
                                                                                                end: 145,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                            kind: False,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 131,
                                                                                end: 146,
                                                                                as_str(): "(true != false)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 109,
                                                            end: 147,
                                                            as_str(): "(__eq(true, false) != (true != false))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 147,
                                                            end: 148,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 151,
                                                                        end: 157,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 158,
                                                                                            end: 162,
                                                                                            as_str(): "__eq",
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
                                                                                            Bool(
                                                                                                LitBool {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 163,
                                                                                                        end: 167,
                                                                                                        as_str(): "true",
                                                                                                    },
                                                                                                    kind: True,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 167,
                                                                                                end: 168,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 169,
                                                                                                    end: 173,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 162,
                                                                                end: 174,
                                                                                as_str(): "(true, true)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 175,
                                                                            end: 177,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: NotEqual {
                                                                                lhs: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 179,
                                                                                                end: 183,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                bang_eq_token: BangEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 184,
                                                                                        end: 186,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                },
                                                                                rhs: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 187,
                                                                                                end: 192,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                            kind: False,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 178,
                                                                                end: 193,
                                                                                as_str(): "(true != false)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 194,
                                                            as_str(): "(__eq(true, true) == (true != false))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 195,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 206,
                                                                                            end: 210,
                                                                                            as_str(): "__eq",
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 211,
                                                                                                        end: 212,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                    parsed: 1,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 212,
                                                                                                end: 213,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 214,
                                                                                                    end: 216,
                                                                                                    as_str(): "22",
                                                                                                },
                                                                                                parsed: 22,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 210,
                                                                                end: 217,
                                                                                as_str(): "(1, 22)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 218,
                                                                            end: 220,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 222,
                                                                                                end: 223,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                double_eq_token: DoubleEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 224,
                                                                                        end: 226,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                rhs: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 227,
                                                                                                end: 229,
                                                                                                as_str(): "22",
                                                                                            },
                                                                                            parsed: 22,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 221,
                                                                                end: 230,
                                                                                as_str(): "(1 == 22)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 205,
                                                            end: 231,
                                                            as_str(): "(__eq(1, 22) == (1 == 22))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 231,
                                                            end: 232,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 235,
                                                                        end: 241,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 242,
                                                                                            end: 246,
                                                                                            as_str(): "__eq",
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 247,
                                                                                                        end: 248,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                    parsed: 1,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 248,
                                                                                                end: 249,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 250,
                                                                                                    end: 251,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 246,
                                                                                end: 252,
                                                                                as_str(): "(1, 1)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 253,
                                                                            end: 255,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 257,
                                                                                                end: 258,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                double_eq_token: DoubleEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 259,
                                                                                        end: 261,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                rhs: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 262,
                                                                                                end: 263,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 256,
                                                                                end: 264,
                                                                                as_str(): "(1 == 1)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 241,
                                                            end: 265,
                                                            as_str(): "(__eq(1, 1) == (1 == 1))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 266,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 273,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 274,
                                                                end: 275,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 275,
                                                                    end: 276,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 277,
                                                                                end: 279,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 280,
                                                            end: 281,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 282,
                                                                    end: 283,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 283,
                                                            end: 284,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 287,
                                                            end: 290,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 291,
                                                                end: 292,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 292,
                                                                    end: 293,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 294,
                                                                                end: 296,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 297,
                                                            end: 298,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 299,
                                                                    end: 301,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 302,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 305,
                                                            end: 308,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 309,
                                                                end: 310,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 310,
                                                                    end: 311,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 312,
                                                                                end: 314,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 315,
                                                            end: 316,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 317,
                                                                    end: 318,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 318,
                                                            end: 319,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 322,
                                                                        end: 328,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 329,
                                                                                            end: 333,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 334,
                                                                                                            end: 335,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 335,
                                                                                                end: 336,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 337,
                                                                                                        end: 338,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 333,
                                                                                end: 339,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 340,
                                                                            end: 342,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 344,
                                                                                                    end: 345,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 346,
                                                                                        end: 348,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 349,
                                                                                                    end: 350,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 343,
                                                                                end: 351,
                                                                                as_str(): "(a == b)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 328,
                                                            end: 352,
                                                            as_str(): "(__eq(a, b) == (a == b))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 352,
                                                            end: 353,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 356,
                                                                        end: 362,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 363,
                                                                                            end: 367,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 368,
                                                                                                            end: 369,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 369,
                                                                                                end: 370,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 371,
                                                                                                        end: 372,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 367,
                                                                                end: 373,
                                                                                as_str(): "(a, c)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 374,
                                                                            end: 376,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 378,
                                                                                                    end: 379,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 380,
                                                                                        end: 382,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 383,
                                                                                                    end: 384,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 377,
                                                                                end: 385,
                                                                                as_str(): "(a == c)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 362,
                                                            end: 386,
                                                            as_str(): "(__eq(a, c) == (a == c))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 386,
                                                            end: 387,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 391,
                                                            end: 394,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 395,
                                                                end: 396,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 396,
                                                                    end: 397,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 398,
                                                                                end: 401,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 402,
                                                            end: 403,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 404,
                                                                    end: 405,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 405,
                                                            end: 406,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 409,
                                                            end: 412,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 413,
                                                                end: 414,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 414,
                                                                    end: 415,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 416,
                                                                                end: 419,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 420,
                                                            end: 421,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 422,
                                                                    end: 424,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 424,
                                                            end: 425,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 428,
                                                            end: 431,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 432,
                                                                end: 433,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 433,
                                                                    end: 434,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 435,
                                                                                end: 438,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 439,
                                                            end: 440,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 441,
                                                                    end: 442,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 442,
                                                            end: 443,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 446,
                                                                        end: 452,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 453,
                                                                                            end: 457,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 458,
                                                                                                            end: 459,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 459,
                                                                                                end: 460,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 461,
                                                                                                        end: 462,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 457,
                                                                                end: 463,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 464,
                                                                            end: 466,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 468,
                                                                                                    end: 469,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 470,
                                                                                        end: 472,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 473,
                                                                                                    end: 474,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 467,
                                                                                end: 475,
                                                                                as_str(): "(a == b)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 452,
                                                            end: 476,
                                                            as_str(): "(__eq(a, b) == (a == b))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 476,
                                                            end: 477,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 480,
                                                                        end: 486,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 487,
                                                                                            end: 491,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 492,
                                                                                                            end: 493,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 493,
                                                                                                end: 494,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 495,
                                                                                                        end: 496,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 491,
                                                                                end: 497,
                                                                                as_str(): "(a, c)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 498,
                                                                            end: 500,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 502,
                                                                                                    end: 503,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 504,
                                                                                        end: 506,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 507,
                                                                                                    end: 508,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 501,
                                                                                end: 509,
                                                                                as_str(): "(a == c)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 486,
                                                            end: 510,
                                                            as_str(): "(__eq(a, c) == (a == c))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 510,
                                                            end: 511,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 515,
                                                            end: 518,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 519,
                                                                end: 520,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 520,
                                                                    end: 521,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 522,
                                                                                end: 525,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 526,
                                                            end: 527,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 528,
                                                                    end: 529,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 529,
                                                            end: 530,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 533,
                                                            end: 536,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 537,
                                                                end: 538,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 538,
                                                                    end: 539,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 540,
                                                                                end: 543,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 544,
                                                            end: 545,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 546,
                                                                    end: 548,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 548,
                                                            end: 549,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 552,
                                                            end: 555,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 556,
                                                                end: 557,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 557,
                                                                    end: 558,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 559,
                                                                                end: 562,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 563,
                                                            end: 564,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 565,
                                                                    end: 566,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 566,
                                                            end: 567,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 570,
                                                                        end: 576,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 577,
                                                                                            end: 581,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 582,
                                                                                                            end: 583,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 583,
                                                                                                end: 584,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 585,
                                                                                                        end: 586,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 581,
                                                                                end: 587,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 588,
                                                                            end: 590,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 592,
                                                                                                    end: 593,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 594,
                                                                                        end: 596,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 597,
                                                                                                    end: 598,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 591,
                                                                                end: 599,
                                                                                as_str(): "(a == b)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 576,
                                                            end: 600,
                                                            as_str(): "(__eq(a, b) == (a == b))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 600,
                                                            end: 601,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 611,
                                                                                            end: 615,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 616,
                                                                                                            end: 617,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 617,
                                                                                                end: 618,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 619,
                                                                                                        end: 620,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 615,
                                                                                end: 621,
                                                                                as_str(): "(a, c)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 622,
                                                                            end: 624,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 626,
                                                                                                    end: 627,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 628,
                                                                                        end: 630,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 631,
                                                                                                    end: 632,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 625,
                                                                                end: 633,
                                                                                as_str(): "(a == c)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 610,
                                                            end: 634,
                                                            as_str(): "(__eq(a, c) == (a == c))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 634,
                                                            end: 635,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 639,
                                                            end: 642,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 643,
                                                                end: 644,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 644,
                                                                    end: 645,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 646,
                                                                                end: 649,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 650,
                                                            end: 651,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 652,
                                                                    end: 653,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 653,
                                                            end: 654,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 657,
                                                            end: 660,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 661,
                                                                end: 662,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 662,
                                                                    end: 663,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 664,
                                                                                end: 667,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 668,
                                                            end: 669,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 670,
                                                                    end: 672,
                                                                    as_str(): "22",
                                                                },
                                                                parsed: 22,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 672,
                                                            end: 673,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 676,
                                                            end: 679,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0e931aa40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                ),
                                                                start: 680,
                                                                end: 681,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 681,
                                                                    end: 682,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 683,
                                                                                end: 686,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 687,
                                                            end: 688,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 689,
                                                                    end: 690,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 690,
                                                            end: 691,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 694,
                                                                        end: 700,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 701,
                                                                                            end: 705,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 706,
                                                                                                            end: 707,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 707,
                                                                                                end: 708,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 709,
                                                                                                        end: 710,
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
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 705,
                                                                                end: 711,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 712,
                                                                            end: 714,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 716,
                                                                                                    end: 717,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 718,
                                                                                        end: 720,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 721,
                                                                                                    end: 722,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 715,
                                                                                end: 723,
                                                                                as_str(): "(a == b)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 700,
                                                            end: 724,
                                                            as_str(): "(__eq(a, b) == (a == b))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 724,
                                                            end: 725,
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
                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 728,
                                                                        end: 734,
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
                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 735,
                                                                                            end: 739,
                                                                                            as_str(): "__eq",
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
                                                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 740,
                                                                                                            end: 741,
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
                                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 741,
                                                                                                end: 742,
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
                                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 743,
                                                                                                        end: 744,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 739,
                                                                                end: 745,
                                                                                as_str(): "(a, c)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0e931aa40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 746,
                                                                            end: 748,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Parens(
                                                                        Parens {
                                                                            inner: Equal {
                                                                                lhs: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 750,
                                                                                                    end: 751,
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
                                                                                        src (ptr): 0x00007fb0e931aa40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 752,
                                                                                        end: 754,
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
                                                                                                    src (ptr): 0x00007fb0e931aa40,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                                    ),
                                                                                                    start: 755,
                                                                                                    end: 756,
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
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0e931aa40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 749,
                                                                                end: 757,
                                                                                as_str(): "(a == c)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 734,
                                                            end: 758,
                                                            as_str(): "(__eq(a, c) == (a == c))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 759,
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
                                                            src (ptr): 0x00007fb0e931aa40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                                            ),
                                                            start: 763,
                                                            end: 764,
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
                                        src (ptr): 0x00007fb0e931aa40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRa7ZmpC/eq_intrinsic/src/main.sw",
                                        ),
                                        start: 52,
                                        end: 766,
                                        as_str(): "{\n\n  assert(__eq(true, true) == (true == true));\n  assert(__eq(true, false) != (true != false));\n  assert(__eq(true, true) == (true != false));\n\n  assert(__eq(1, 22) == (1 == 22));\n  assert(__eq(1, 1) == (1 == 1));\n\n  let a: u8 = 1;\n  let b: u8 = 22;\n  let c: u8 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u16 = 1;\n  let b: u16 = 22;\n  let c: u16 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u32 = 1;\n  let b: u32 = 22;\n  let c: u32 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  let a: u64 = 1;\n  let b: u64 = 22;\n  let c: u64 = 1;\n  assert(__eq(a, b) == (a == b));\n  assert(__eq(a, c) == (a == c));\n\n  2\n}",
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
