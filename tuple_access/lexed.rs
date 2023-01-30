Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe032b8a4f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe032b8a4f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 24,
                                            as_str(): "gimme_a_pair",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 24,
                                            end: 26,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 27,
                                                    end: 29,
                                                    as_str(): "->",
                                                },
                                            },
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
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
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 35,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathType {
                                                                        root_opt: None,
                                                                        prefix: PathTypeSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 36,
                                                                                    end: 39,
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
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 30,
                                                        end: 40,
                                                        as_str(): "(u32, u64)",
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: Some(
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 48,
                                                                        end: 49,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: Some(
                                                                        (
                                                                            U32,
                                                                            Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 49,
                                                                                end: 52,
                                                                                as_str(): "u32",
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 52,
                                                                end: 53,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 54,
                                                                                end: 55,
                                                                                as_str(): "2",
                                                                            },
                                                                            parsed: 2,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                        ),
                                                                                        start: 55,
                                                                                        end: 58,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 47,
                                                        end: 59,
                                                        as_str(): "(1u32, 2u64)",
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 61,
                                        as_str(): "{\n    (1u32, 2u64)\n}",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 63,
                                            end: 65,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 70,
                                            as_str(): "test",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 70,
                                                        end: 71,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                    ),
                                                                    start: 71,
                                                                    end: 72,
                                                                    as_str(): "T",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                    ),
                                                                    start: 72,
                                                                    end: 73,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 75,
                                                                as_str(): "E",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe032b8a4f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                        ),
                                                        start: 75,
                                                        end: 76,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
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
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 77,
                                                                        end: 78,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                    ),
                                                                    start: 78,
                                                                    end: 79,
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
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 80,
                                                                                end: 81,
                                                                                as_str(): "T",
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
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 82,
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
                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                    ),
                                                                    start: 83,
                                                                    end: 84,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 84,
                                                                end: 85,
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
                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                            ),
                                                                            start: 86,
                                                                            end: 87,
                                                                            as_str(): "E",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 76,
                                            end: 88,
                                            as_str(): "(a: T, b: E)",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 95,
                                                            end: 98,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Tuple(
                                                        Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        Var {
                                                                            reference: None,
                                                                            mutable: None,
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 100,
                                                                                    end: 101,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 101,
                                                                                end: 102,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 103,
                                                                                end: 104,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 99,
                                                                end: 105,
                                                                as_str(): "(x, y)",
                                                            },
                                                        },
                                                    ),
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 106,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Tuple(
                                                                Parens {
                                                                    inner: Cons {
                                                                        head: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                            ),
                                                                                            start: 108,
                                                                                            end: 109,
                                                                                            as_str(): "T",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                        tail: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Path(
                                                                                    PathType {
                                                                                        root_opt: None,
                                                                                        prefix: PathTypeSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                    ),
                                                                                                    start: 111,
                                                                                                    end: 112,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [],
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 107,
                                                                        end: 113,
                                                                        as_str(): "(T, E)",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 115,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 117,
                                                                                    end: 118,
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
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 118,
                                                                        end: 119,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                            ),
                                                                                            start: 120,
                                                                                            end: 121,
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
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 122,
                                                                as_str(): "(a, b)",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 122,
                                                            end: 123,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 89,
                                        end: 125,
                                        as_str(): "{\n    let (x, y): (T, E) = (a, b);\n}",
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 130,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 131,
                                            end: 135,
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
                                            src (ptr): 0x00007fe032b8a4f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                            ),
                                            start: 135,
                                            end: 137,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe032b8a4f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                    ),
                                                    start: 138,
                                                    end: 140,
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
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 141,
                                                                end: 144,
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
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 151,
                                                            end: 154,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Tuple(
                                                        Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        Var {
                                                                            reference: None,
                                                                            mutable: None,
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 156,
                                                                                    end: 159,
                                                                                    as_str(): "foo",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 160,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 161,
                                                                                end: 164,
                                                                                as_str(): "bar",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 155,
                                                                end: 165,
                                                                as_str(): "(foo, bar)",
                                                            },
                                                        },
                                                    ),
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 166,
                                                            end: 167,
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
                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                            ),
                                                                            start: 168,
                                                                            end: 180,
                                                                            as_str(): "gimme_a_pair",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 182,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 183,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 188,
                                                            end: 191,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Tuple(
                                                        Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        Var {
                                                                            reference: None,
                                                                            mutable: None,
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 193,
                                                                                    end: 194,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 194,
                                                                                end: 195,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    Var {
                                                                        reference: None,
                                                                        mutable: None,
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 196,
                                                                                end: 197,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 192,
                                                                end: 198,
                                                                as_str(): "(x, y)",
                                                            },
                                                        },
                                                    ),
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                    ),
                                                                    start: 198,
                                                                    end: 199,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Tuple(
                                                                Parens {
                                                                    inner: Cons {
                                                                        head: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                            ),
                                                                                            start: 201,
                                                                                            end: 204,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 204,
                                                                                end: 205,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                        tail: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Path(
                                                                                    PathType {
                                                                                        root_opt: None,
                                                                                        prefix: PathTypeSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                    ),
                                                                                                    start: 206,
                                                                                                    end: 210,
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
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 200,
                                                                        end: 211,
                                                                        as_str(): "(u32, bool)",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 212,
                                                            end: 213,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 215,
                                                                                end: 217,
                                                                                as_str(): "10",
                                                                            },
                                                                            parsed: 10,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 217,
                                                                        end: 218,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                        ),
                                                                                        start: 219,
                                                                                        end: 223,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 214,
                                                                end: 224,
                                                                as_str(): "(10, true)",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 224,
                                                            end: 225,
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
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 305,
                                                                        end: 309,
                                                                        as_str(): "test",
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
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 310,
                                                                                    end: 314,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                            ),
                                                                            start: 314,
                                                                            end: 315,
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
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 316,
                                                                                end: 321,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 309,
                                                            end: 322,
                                                            as_str(): "(true, false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 322,
                                                            end: 323,
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
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 328,
                                                                        end: 332,
                                                                        as_str(): "test",
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
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 334,
                                                                                    end: 336,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                            ),
                                                                            start: 336,
                                                                            end: 337,
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
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 338,
                                                                                end: 340,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 333,
                                                            end: 341,
                                                            as_str(): "(42, 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 341,
                                                            end: 342,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 350,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Tuple(
                                                        Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        Var {
                                                                            reference: None,
                                                                            mutable: None,
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 352,
                                                                                    end: 353,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 353,
                                                                                end: 354,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    Tuple(
                                                                        Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Var {
                                                                                            reference: None,
                                                                                            mutable: None,
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                    ),
                                                                                                    start: 356,
                                                                                                    end: 357,
                                                                                                    as_str(): "b",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                ),
                                                                                                start: 357,
                                                                                                end: 358,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Punctuated {
                                                                                                value_separator_pairs: [
                                                                                                    (
                                                                                                        Var {
                                                                                                            reference: None,
                                                                                                            mutable: None,
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 360,
                                                                                                                    end: 361,
                                                                                                                    as_str(): "c",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                ),
                                                                                                                start: 361,
                                                                                                                end: 362,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    Var {
                                                                                                        reference: None,
                                                                                                        mutable: None,
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                ),
                                                                                                                start: 363,
                                                                                                                end: 364,
                                                                                                                as_str(): "d",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                ),
                                                                                                start: 359,
                                                                                                end: 365,
                                                                                                as_str(): "(c, d)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 355,
                                                                                end: 367,
                                                                                as_str(): "(b, (c, d) )",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 351,
                                                                end: 369,
                                                                as_str(): "(a, (b, (c, d) ) )",
                                                            },
                                                        },
                                                    ),
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                    ),
                                                                    start: 369,
                                                                    end: 370,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Tuple(
                                                                Parens {
                                                                    inner: Cons {
                                                                        head: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                            ),
                                                                                            start: 372,
                                                                                            end: 375,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 375,
                                                                                end: 376,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                        tail: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Tuple(
                                                                                    Parens {
                                                                                        inner: Cons {
                                                                                            head: Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                ),
                                                                                                                start: 378,
                                                                                                                end: 381,
                                                                                                                as_str(): "u32",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [],
                                                                                                },
                                                                                            ),
                                                                                            comma_token: CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                    ),
                                                                                                    start: 381,
                                                                                                    end: 382,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                            tail: Punctuated {
                                                                                                value_separator_pairs: [],
                                                                                                final_value_opt: Some(
                                                                                                    Tuple(
                                                                                                        Parens {
                                                                                                            inner: Cons {
                                                                                                                head: Path(
                                                                                                                    PathType {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathTypeSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 384,
                                                                                                                                    end: 388,
                                                                                                                                    as_str(): "bool",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            generics_opt: None,
                                                                                                                        },
                                                                                                                        suffix: [],
                                                                                                                    },
                                                                                                                ),
                                                                                                                comma_token: CommaToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 388,
                                                                                                                        end: 389,
                                                                                                                        as_str(): ",",
                                                                                                                    },
                                                                                                                },
                                                                                                                tail: Punctuated {
                                                                                                                    value_separator_pairs: [],
                                                                                                                    final_value_opt: Some(
                                                                                                                        Str {
                                                                                                                            str_token: StrToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 390,
                                                                                                                                    end: 393,
                                                                                                                                    as_str(): "str",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            length: SquareBrackets {
                                                                                                                                inner: Literal(
                                                                                                                                    Int(
                                                                                                                                        LitInt {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 394,
                                                                                                                                                end: 395,
                                                                                                                                                as_str(): "2",
                                                                                                                                            },
                                                                                                                                            parsed: 2,
                                                                                                                                            ty_opt: None,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 393,
                                                                                                                                    end: 396,
                                                                                                                                    as_str(): "[2]",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                },
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                ),
                                                                                                                start: 383,
                                                                                                                end: 397,
                                                                                                                as_str(): "(bool, str[2])",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                            ),
                                                                                            start: 377,
                                                                                            end: 399,
                                                                                            as_str(): "(u32, (bool, str[2]) )",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 371,
                                                                        end: 401,
                                                                        as_str(): "(u64, (u32, (bool, str[2]) ) )",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 402,
                                                            end: 403,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                ),
                                                                                start: 405,
                                                                                end: 407,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                        ),
                                                                                        start: 407,
                                                                                        end: 410,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                        ),
                                                                        start: 410,
                                                                        end: 411,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Tuple(
                                                                            Parens {
                                                                                inner: Cons {
                                                                                    head: Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                    ),
                                                                                                    start: 413,
                                                                                                    end: 415,
                                                                                                    as_str(): "42",
                                                                                                },
                                                                                                parsed: 42,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U32,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                            ),
                                                                                                            start: 415,
                                                                                                            end: 418,
                                                                                                            as_str(): "u32",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    comma_token: CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe032b8a4f0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                            ),
                                                                                            start: 418,
                                                                                            end: 419,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                    tail: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            Tuple(
                                                                                                Parens {
                                                                                                    inner: Cons {
                                                                                                        head: Literal(
                                                                                                            Bool(
                                                                                                                LitBool {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 421,
                                                                                                                        end: 425,
                                                                                                                        as_str(): "true",
                                                                                                                    },
                                                                                                                    kind: True,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                        comma_token: CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                ),
                                                                                                                start: 425,
                                                                                                                end: 426,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                        tail: Punctuated {
                                                                                                            value_separator_pairs: [],
                                                                                                            final_value_opt: Some(
                                                                                                                Literal(
                                                                                                                    String(
                                                                                                                        LitString {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 427,
                                                                                                                                end: 431,
                                                                                                                                as_str(): "\"ok\"",
                                                                                                                            },
                                                                                                                            parsed: "ok",
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe032b8a4f0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                                        ),
                                                                                                        start: 420,
                                                                                                        end: 432,
                                                                                                        as_str(): "(true, \"ok\")",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe032b8a4f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                                    ),
                                                                                    start: 412,
                                                                                    end: 434,
                                                                                    as_str(): "(42u32, (true, \"ok\") )",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 404,
                                                                end: 436,
                                                                as_str(): "(42u64, (42u32, (true, \"ok\") ) )",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe032b8a4f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                            ),
                                                            start: 436,
                                                            end: 437,
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
                                                                src (ptr): 0x00007fe032b8a4f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                                                ),
                                                                start: 442,
                                                                end: 443,
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
                                        src (ptr): 0x00007fe032b8a4f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRU9mGAk/tuple_access/src/main.sw",
                                        ),
                                        start: 145,
                                        end: 445,
                                        as_str(): "{\n    let (foo, bar) = gimme_a_pair();\n    let (x, y): (u32, bool) = (10, true);\n    //let (x, y): (u32, _) = (42, true); // this generates a parsing error\n    test(true, false);\n    test (42, 42);\n    let (a, (b, (c, d) ) ): (u64, (u32, (bool, str[2]) ) ) = (42u64, (42u32, (true, \"ok\") ) );\n    a\n}",
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
