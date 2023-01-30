Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe03346fb00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe03346fb00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                        src (ptr): 0x00007fe03346fb00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                        src (ptr): 0x00007fe03346fb00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                                        src (ptr): 0x00007fe03346fb00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                        src (ptr): 0x00007fe03346fb00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
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
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 63,
                                            end: 65,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 70,
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
                                            src (ptr): 0x00007fe03346fb00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                            ),
                                            start: 70,
                                            end: 72,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe03346fb00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                    ),
                                                    start: 73,
                                                    end: 75,
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
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 79,
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
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 89,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 91,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 92,
                                                            end: 93,
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
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 94,
                                                                            end: 106,
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
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 108,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 109,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 117,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 119,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 120,
                                                            end: 121,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 127,
                                                                as_str(): "match",
                                                            },
                                                        },
                                                        value: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 128,
                                                                            end: 129,
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
                                                        branches: Braces {
                                                            inner: [
                                                                MatchBranch {
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
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 141,
                                                                                                    end: 142,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 142,
                                                                                                end: 143,
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
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 144,
                                                                                                    end: 145,
                                                                                                    as_str(): "3",
                                                                                                },
                                                                                                parsed: 3,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 145,
                                                                                                            end: 148,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                ),
                                                                                start: 140,
                                                                                end: 149,
                                                                                as_str(): "(a, 3u64)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 150,
                                                                            end: 152,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Cons {
                                                                                                head: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 156,
                                                                                                                    end: 157,
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
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 157,
                                                                                                        end: 158,
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
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 159,
                                                                                                                        end: 160,
                                                                                                                        as_str(): "7",
                                                                                                                    },
                                                                                                                    parsed: 7,
                                                                                                                    ty_opt: Some(
                                                                                                                        (
                                                                                                                            U32,
                                                                                                                            Span {
                                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 160,
                                                                                                                                end: 163,
                                                                                                                                as_str(): "u32",
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
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 155,
                                                                                                end: 164,
                                                                                                as_str(): "(a, 7u32)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                ),
                                                                                start: 153,
                                                                                end: 166,
                                                                                as_str(): "{ (a, 7u32) }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 166,
                                                                                    end: 167,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
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
                                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 177,
                                                                                                    end: 178,
                                                                                                    as_str(): "a",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 178,
                                                                                                end: 179,
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
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 180,
                                                                                                end: 181,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                ),
                                                                                start: 176,
                                                                                end: 182,
                                                                                as_str(): "(a, b)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 183,
                                                                            end: 185,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
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
                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 189,
                                                                                                                end: 190,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                            parsed: 0,
                                                                                                            ty_opt: Some(
                                                                                                                (
                                                                                                                    U32,
                                                                                                                    Span {
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 190,
                                                                                                                        end: 193,
                                                                                                                        as_str(): "u32",
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 193,
                                                                                                        end: 194,
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
                                                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 195,
                                                                                                                        end: 196,
                                                                                                                        as_str(): "9",
                                                                                                                    },
                                                                                                                    parsed: 9,
                                                                                                                    ty_opt: Some(
                                                                                                                        (
                                                                                                                            U32,
                                                                                                                            Span {
                                                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 196,
                                                                                                                                end: 199,
                                                                                                                                as_str(): "u32",
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
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 188,
                                                                                                end: 200,
                                                                                                as_str(): "(0u32, 9u32)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                ),
                                                                                start: 186,
                                                                                end: 202,
                                                                                as_str(): "{ (0u32, 9u32) }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03346fb00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                    ),
                                                                                    start: 202,
                                                                                    end: 203,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe03346fb00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                ),
                                                                start: 130,
                                                                end: 209,
                                                                as_str(): "{\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03346fb00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                            ),
                                                            start: 209,
                                                            end: 210,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe03346fb00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                        ),
                                                        start: 215,
                                                        end: 220,
                                                        as_str(): "match",
                                                    },
                                                },
                                                value: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 221,
                                                                    end: 222,
                                                                    as_str(): "y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                        incomplete_suffix: false,
                                                    },
                                                ),
                                                branches: Braces {
                                                    inner: [
                                                        MatchBranch {
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
                                                                                            src (ptr): 0x00007fe03346fb00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                            ),
                                                                                            start: 234,
                                                                                            end: 235,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 235,
                                                                                        end: 236,
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
                                                                                        src (ptr): 0x00007fe03346fb00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                        ),
                                                                                        start: 237,
                                                                                        end: 238,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe03346fb00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                        ),
                                                                        start: 233,
                                                                        end: 239,
                                                                        as_str(): "(a, b)",
                                                                    },
                                                                },
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03346fb00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                    ),
                                                                    start: 240,
                                                                    end: 242,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Block {
                                                                block: Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe03346fb00,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                                                ),
                                                                                                start: 245,
                                                                                                end: 246,
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
                                                                        src (ptr): 0x00007fe03346fb00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                        ),
                                                                        start: 243,
                                                                        end: 248,
                                                                        as_str(): "{ b }",
                                                                    },
                                                                },
                                                                comma_token_opt: Some(
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03346fb00,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                                            ),
                                                                            start: 248,
                                                                            end: 249,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe03346fb00,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                                        ),
                                                        start: 223,
                                                        end: 255,
                                                        as_str(): "{\n        (a, b) => { b },\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe03346fb00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRcvaijd/tuple_desugaring/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 257,
                                        as_str(): "{\n    let x = gimme_a_pair();\n    let y = match x {\n        (a, 3u64) => { (a, 7u32) },\n        (a, b) => { (0u32, 9u32) },\n    };\n    match y {\n        (a, b) => { b },\n    }\n}",
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
