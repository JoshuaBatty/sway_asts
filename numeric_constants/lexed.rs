Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe06890a930,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe06890a930,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                src (ptr): 0x00007fe06890a930,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe06890a930,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                    src (ptr): 0x00007fe06890a930,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                            src (ptr): 0x00007fe06890a930,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                    src (ptr): 0x00007fe06890a930,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
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
                                                                src (ptr): 0x00007fe06890a930,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                ),
                                                                start: 48,
                                                                end: 52,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 59,
                                                                        end: 65,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 66,
                                                                                            end: 69,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 69,
                                                                                                end: 71,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 71,
                                                                                                    end: 74,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 74,
                                                                                end: 76,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 77,
                                                                            end: 79,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 80,
                                                                                    end: 100,
                                                                                    as_str(): "18446744073709551615",
                                                                                },
                                                                                parsed: 18446744073709551615,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 65,
                                                            end: 101,
                                                            as_str(): "(u64::max() == 18446744073709551615)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 102,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 107,
                                                                        end: 113,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 114,
                                                                                            end: 117,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 117,
                                                                                                end: 119,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 119,
                                                                                                    end: 122,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 122,
                                                                                end: 124,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 125,
                                                                            end: 127,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 128,
                                                                                    end: 129,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 129,
                                                                                            end: 132,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 133,
                                                            as_str(): "(u64::min() == 0u64)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 133,
                                                            end: 134,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 139,
                                                                        end: 145,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 146,
                                                                                            end: 149,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 149,
                                                                                                end: 151,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 151,
                                                                                                    end: 155,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 155,
                                                                                end: 157,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 158,
                                                                            end: 160,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 161,
                                                                                    end: 163,
                                                                                    as_str(): "64",
                                                                                },
                                                                                parsed: 64,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 163,
                                                                                            end: 166,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 145,
                                                            end: 167,
                                                            as_str(): "(u64::bits() == 64u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 168,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 173,
                                                                        end: 179,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 180,
                                                                                            end: 183,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 183,
                                                                                                end: 185,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 185,
                                                                                                    end: 188,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 188,
                                                                                end: 190,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 191,
                                                                            end: 193,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 194,
                                                                                    end: 204,
                                                                                    as_str(): "4294967295",
                                                                                },
                                                                                parsed: 4294967295,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 204,
                                                                                            end: 207,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 179,
                                                            end: 208,
                                                            as_str(): "(u32::max() == 4294967295u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 209,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 214,
                                                                        end: 220,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 221,
                                                                                            end: 224,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 224,
                                                                                                end: 226,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 226,
                                                                                                    end: 229,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 229,
                                                                                end: 231,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 232,
                                                                            end: 234,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 235,
                                                                                    end: 236,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 236,
                                                                                            end: 239,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 220,
                                                            end: 240,
                                                            as_str(): "(u32::min() == 0u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 240,
                                                            end: 241,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 246,
                                                                        end: 252,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 253,
                                                                                            end: 256,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 256,
                                                                                                end: 258,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 258,
                                                                                                    end: 262,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 262,
                                                                                end: 264,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 267,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 268,
                                                                                    end: 270,
                                                                                    as_str(): "32",
                                                                                },
                                                                                parsed: 32,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 270,
                                                                                            end: 273,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 252,
                                                            end: 274,
                                                            as_str(): "(u32::bits() == 32u16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 275,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 280,
                                                                        end: 286,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 287,
                                                                                            end: 290,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 290,
                                                                                                end: 292,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 292,
                                                                                                    end: 295,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 295,
                                                                                end: 297,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 298,
                                                                            end: 300,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 301,
                                                                                    end: 306,
                                                                                    as_str(): "65535",
                                                                                },
                                                                                parsed: 65535,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 306,
                                                                                            end: 309,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 310,
                                                            as_str(): "(u16::max() == 65535u16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 311,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 316,
                                                                        end: 322,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 323,
                                                                                            end: 326,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 326,
                                                                                                end: 328,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 328,
                                                                                                    end: 331,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 331,
                                                                                end: 333,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 334,
                                                                            end: 336,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 337,
                                                                                    end: 338,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 338,
                                                                                            end: 341,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 322,
                                                            end: 342,
                                                            as_str(): "(u16::min() == 0u16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 342,
                                                            end: 343,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 348,
                                                                        end: 354,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 355,
                                                                                            end: 358,
                                                                                            as_str(): "u16",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 358,
                                                                                                end: 360,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 360,
                                                                                                    end: 364,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 364,
                                                                                end: 366,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 367,
                                                                            end: 369,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 370,
                                                                                    end: 372,
                                                                                    as_str(): "16",
                                                                                },
                                                                                parsed: 16,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 372,
                                                                                            end: 374,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 375,
                                                            as_str(): "(u16::bits() == 16u8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 375,
                                                            end: 376,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 381,
                                                                        end: 387,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 388,
                                                                                            end: 390,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 390,
                                                                                                end: 392,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 392,
                                                                                                    end: 395,
                                                                                                    as_str(): "max",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 395,
                                                                                end: 397,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 398,
                                                                            end: 400,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 401,
                                                                                    end: 404,
                                                                                    as_str(): "255",
                                                                                },
                                                                                parsed: 255,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 404,
                                                                                            end: 406,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 387,
                                                            end: 407,
                                                            as_str(): "(u8::max() == 255u8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 407,
                                                            end: 408,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 413,
                                                                        end: 419,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 420,
                                                                                            end: 422,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 422,
                                                                                                end: 424,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 424,
                                                                                                    end: 427,
                                                                                                    as_str(): "min",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 427,
                                                                                end: 429,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 430,
                                                                            end: 432,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 433,
                                                                                    end: 434,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 434,
                                                                                            end: 436,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 419,
                                                            end: 437,
                                                            as_str(): "(u8::min() == 0u8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 437,
                                                            end: 438,
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
                                                                        src (ptr): 0x00007fe06890a930,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                        ),
                                                                        start: 443,
                                                                        end: 449,
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
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 450,
                                                                                            end: 452,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe06890a930,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                ),
                                                                                                start: 452,
                                                                                                end: 454,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                                    ),
                                                                                                    start: 454,
                                                                                                    end: 458,
                                                                                                    as_str(): "bits",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06890a930,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                ),
                                                                                start: 458,
                                                                                end: 460,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06890a930,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                            ),
                                                                            start: 461,
                                                                            end: 463,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe06890a930,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                    ),
                                                                                    start: 464,
                                                                                    end: 465,
                                                                                    as_str(): "8",
                                                                                },
                                                                                parsed: 8,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe06890a930,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                                                            ),
                                                                                            start: 465,
                                                                                            end: 468,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 449,
                                                            end: 469,
                                                            as_str(): "(u8::bits() == 8u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 469,
                                                            end: 470,
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
                                                            src (ptr): 0x00007fe06890a930,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                                            ),
                                                            start: 476,
                                                            end: 480,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe06890a930,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8YbIZ/numeric_constants/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 482,
                                        as_str(): "{\n    assert(u64::max() == 18446744073709551615);\n    assert(u64::min() == 0u64);\n    assert(u64::bits() == 64u32);\n    assert(u32::max() == 4294967295u32);\n    assert(u32::min() == 0u32);\n    assert(u32::bits() == 32u16);\n    assert(u16::max() == 65535u16);\n    assert(u16::min() == 0u16);\n    assert(u16::bits() == 16u8);\n    assert(u8::max() == 255u8);\n    assert(u8::min() == 0u8);\n    assert(u8::bits() == 8u32);\n\n    true\n}",
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
