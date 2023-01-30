Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb12632e1f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb12632e1f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                            src (ptr): 0x00007fb12632e1f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                    src (ptr): 0x00007fb12632e1f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                                src (ptr): 0x00007fb12632e1f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 26,
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
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 33,
                                                                        end: 39,
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
                                                                    lhs: Not {
                                                                        bang_token: BangToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                ),
                                                                                start: 40,
                                                                                end: 41,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 41,
                                                                                        end: 42,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U8,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                ),
                                                                                                start: 42,
                                                                                                end: 44,
                                                                                                as_str(): "u8",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 45,
                                                                            end: 47,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 48,
                                                                                    end: 51,
                                                                                    as_str(): "253",
                                                                                },
                                                                                parsed: 253,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 51,
                                                                                            end: 53,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 39,
                                                            end: 54,
                                                            as_str(): "(!2u8 == 253u8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 54,
                                                            end: 55,
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
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 60,
                                                                        end: 66,
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
                                                                    lhs: Not {
                                                                        bang_token: BangToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                ),
                                                                                start: 67,
                                                                                end: 68,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 68,
                                                                                        end: 69,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U16,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                ),
                                                                                                start: 69,
                                                                                                end: 72,
                                                                                                as_str(): "u16",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                            ),
                                                                            start: 73,
                                                                            end: 75,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 76,
                                                                                    end: 81,
                                                                                    as_str(): "65533",
                                                                                },
                                                                                parsed: 65533,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U16,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 84,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 66,
                                                            end: 85,
                                                            as_str(): "(!2u16 == 65533u16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 85,
                                                            end: 86,
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
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 91,
                                                                        end: 97,
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
                                                                    lhs: Not {
                                                                        bang_token: BangToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 99,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 99,
                                                                                        end: 100,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U32,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                ),
                                                                                                start: 100,
                                                                                                end: 103,
                                                                                                as_str(): "u32",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 107,
                                                                                    end: 117,
                                                                                    as_str(): "4294967293",
                                                                                },
                                                                                parsed: 4294967293,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 117,
                                                                                            end: 120,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 97,
                                                            end: 121,
                                                            as_str(): "(!2u32 == 4294967293u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 121,
                                                            end: 122,
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
                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                        ),
                                                                        start: 127,
                                                                        end: 133,
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
                                                                    lhs: Not {
                                                                        bang_token: BangToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                ),
                                                                                start: 134,
                                                                                end: 135,
                                                                                as_str(): "!",
                                                                            },
                                                                        },
                                                                        expr: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb12632e1f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                        ),
                                                                                        start: 135,
                                                                                        end: 136,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U64,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb12632e1f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                                ),
                                                                                                start: 136,
                                                                                                end: 139,
                                                                                                as_str(): "u64",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
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
                                                                                    src (ptr): 0x00007fb12632e1f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                    ),
                                                                                    start: 143,
                                                                                    end: 163,
                                                                                    as_str(): "18446744073709551613",
                                                                                },
                                                                                parsed: 18446744073709551613,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb12632e1f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                                                            ),
                                                                                            start: 163,
                                                                                            end: 166,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 133,
                                                            end: 167,
                                                            as_str(): "(!2u64 == 18446744073709551613u64)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 168,
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
                                                            src (ptr): 0x00007fb12632e1f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                                            ),
                                                            start: 174,
                                                            end: 178,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb12632e1f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRT2ZcvK/bitwise_not/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 180,
                                        as_str(): "{\n    assert(!2u8 == 253u8);\n    assert(!2u16 == 65533u16);\n    assert(!2u32 == 4294967293u32);\n    assert(!2u64 == 18446744073709551613u64);\n\n    true\n}",
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
