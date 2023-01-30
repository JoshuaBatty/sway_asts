Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe082cf6db0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                                src (ptr): 0x00007fe082cf6db0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe082cf6db0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 62,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 75,
                                                                as_str(): "uint64_test1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 75,
                                                                    end: 76,
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 77,
                                                                                end: 80,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 81,
                                                            end: 82,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 83,
                                                                    end: 95,
                                                                    as_str(): "100000000000",
                                                                },
                                                                parsed: 100000000000,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 95,
                                                            end: 96,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 104,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 117,
                                                                as_str(): "uint32_test1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 118,
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 119,
                                                                                end: 122,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 123,
                                                            end: 124,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 125,
                                                                    end: 135,
                                                                    as_str(): "1000000000",
                                                                },
                                                                parsed: 1000000000,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 136,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 141,
                                                            end: 144,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 145,
                                                                end: 157,
                                                                as_str(): "uint16_test1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 157,
                                                                    end: 158,
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 162,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 164,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 165,
                                                                    end: 170,
                                                                    as_str(): "10000",
                                                                },
                                                                parsed: 10000,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 171,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 176,
                                                            end: 179,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 191,
                                                                as_str(): "uint8_test1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 191,
                                                                    end: 192,
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 193,
                                                                                end: 195,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 196,
                                                            end: 197,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 198,
                                                                    end: 201,
                                                                    as_str(): "100",
                                                                },
                                                                parsed: 100,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 201,
                                                            end: 202,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 252,
                                                                        end: 258,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 259,
                                                                                            end: 271,
                                                                                            as_str(): "uint64_test1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 272,
                                                                                end: 273,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 274,
                                                                                        end: 277,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U64,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 277,
                                                                                                end: 280,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 281,
                                                                            end: 283,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 284,
                                                                                    end: 285,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 258,
                                                            end: 286,
                                                            as_str(): "(uint64_test1 % 100u64 == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 287,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 292,
                                                                        end: 298,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 299,
                                                                                            end: 311,
                                                                                            as_str(): "uint32_test1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 312,
                                                                                end: 313,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 314,
                                                                                        end: 317,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U32,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 317,
                                                                                                end: 320,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 321,
                                                                            end: 323,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 324,
                                                                                    end: 325,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 326,
                                                            as_str(): "(uint32_test1 % 100u32 == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 326,
                                                            end: 327,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 332,
                                                                        end: 338,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 339,
                                                                                            end: 351,
                                                                                            as_str(): "uint16_test1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 352,
                                                                                end: 353,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 354,
                                                                                        end: 357,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U16,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 357,
                                                                                                end: 360,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 361,
                                                                            end: 363,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 364,
                                                                                    end: 365,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 338,
                                                            end: 366,
                                                            as_str(): "(uint16_test1 % 100u16 == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 366,
                                                            end: 367,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 372,
                                                                        end: 378,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 379,
                                                                                            end: 390,
                                                                                            as_str(): "uint8_test1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 391,
                                                                                end: 392,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 393,
                                                                                        end: 396,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U8,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 396,
                                                                                                end: 398,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 399,
                                                                            end: 401,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 402,
                                                                                    end: 403,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 378,
                                                            end: 404,
                                                            as_str(): "(uint8_test1 % 100u8 == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 404,
                                                            end: 405,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 411,
                                                            end: 414,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 415,
                                                                end: 427,
                                                                as_str(): "uint64_test2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 427,
                                                                    end: 428,
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 429,
                                                                                end: 432,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 435,
                                                                    end: 447,
                                                                    as_str(): "100000000005",
                                                                },
                                                                parsed: 100000000005,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 447,
                                                            end: 448,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 453,
                                                            end: 456,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 457,
                                                                end: 469,
                                                                as_str(): "uint32_test2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 469,
                                                                    end: 470,
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 471,
                                                                                end: 474,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 475,
                                                            end: 476,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 477,
                                                                    end: 487,
                                                                    as_str(): "1000000005",
                                                                },
                                                                parsed: 1000000005,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 487,
                                                            end: 488,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 493,
                                                            end: 496,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 497,
                                                                end: 509,
                                                                as_str(): "uint16_test2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 509,
                                                                    end: 510,
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 511,
                                                                                end: 514,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 515,
                                                            end: 516,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 517,
                                                                    end: 522,
                                                                    as_str(): "10005",
                                                                },
                                                                parsed: 10005,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 522,
                                                            end: 523,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 528,
                                                            end: 531,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 532,
                                                                end: 543,
                                                                as_str(): "uint8_test2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 545,
                                                                                end: 547,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 548,
                                                            end: 549,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                    ),
                                                                    start: 550,
                                                                    end: 553,
                                                                    as_str(): "105",
                                                                },
                                                                parsed: 105,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 611,
                                                                        end: 617,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 618,
                                                                                            end: 630,
                                                                                            as_str(): "uint64_test2",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 631,
                                                                                end: 632,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 633,
                                                                                        end: 636,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U64,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 636,
                                                                                                end: 639,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 640,
                                                                            end: 642,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 643,
                                                                                    end: 644,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 645,
                                                            as_str(): "(uint64_test2 % 100u64 == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 645,
                                                            end: 646,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 651,
                                                                        end: 657,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 658,
                                                                                            end: 670,
                                                                                            as_str(): "uint32_test2",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 671,
                                                                                end: 672,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 673,
                                                                                        end: 676,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U32,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 676,
                                                                                                end: 679,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 680,
                                                                            end: 682,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 683,
                                                                                    end: 684,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 657,
                                                            end: 685,
                                                            as_str(): "(uint32_test2 % 100u32 == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 685,
                                                            end: 686,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 691,
                                                                        end: 697,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 698,
                                                                                            end: 710,
                                                                                            as_str(): "uint16_test2",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 711,
                                                                                end: 712,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 713,
                                                                                        end: 716,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U16,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 716,
                                                                                                end: 719,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 720,
                                                                            end: 722,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 723,
                                                                                    end: 724,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 697,
                                                            end: 725,
                                                            as_str(): "(uint16_test2 % 100u16 == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 725,
                                                            end: 726,
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
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 731,
                                                                        end: 737,
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
                                                                    lhs: Modulo {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 738,
                                                                                            end: 749,
                                                                                            as_str(): "uint8_test2",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        percent_token: PercentToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                ),
                                                                                start: 750,
                                                                                end: 751,
                                                                                as_str(): "%",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 752,
                                                                                        end: 755,
                                                                                        as_str(): "100",
                                                                                    },
                                                                                    parsed: 100,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U8,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe082cf6db0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                ),
                                                                                                start: 755,
                                                                                                end: 757,
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
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 758,
                                                                            end: 760,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 761,
                                                                                    end: 762,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 737,
                                                            end: 763,
                                                            as_str(): "(uint8_test2 % 100u8 == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 763,
                                                            end: 764,
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
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 770,
                                                            end: 774,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe082cf6db0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 776,
                                        as_str(): "{\n    let uint64_test1: u64 = 100000000000;\n    let uint32_test1: u32 = 1000000000;\n    let uint16_test1: u16 = 10000;\n    let uint8_test1: u8 = 100;\n\n    // Ensure 0 remainder returns correctly\n    assert(uint64_test1 % 100u64 == 0);\n    assert(uint32_test1 % 100u32 == 0);\n    assert(uint16_test1 % 100u16 == 0);\n    assert(uint8_test1 % 100u8 == 0);\n\n    let uint64_test2: u64 = 100000000005;\n    let uint32_test2: u32 = 1000000005;\n    let uint16_test2: u16 = 10005;\n    let uint8_test2: u8 = 105;\n\n    // Ensure non zero remainder returns correctly\n    assert(uint64_test2 % 100u64 == 5);\n    assert(uint32_test2 % 100u32 == 5);\n    assert(uint16_test2 % 100u16 == 5);\n    assert(uint8_test2 % 100u8 == 5);\n\n    true\n}",
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
