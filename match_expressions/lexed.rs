Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0c8b9aa10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0c8b9aa10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
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
                                            src (ptr): 0x00007fe0c8b9aa10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c8b9aa10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
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
                                            src (ptr): 0x00007fe0c8b9aa10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
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
                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
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
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 25,
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
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 32,
                                                            end: 35,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 36,
                                                                end: 37,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 38,
                                                            end: 39,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 40,
                                                                    end: 41,
                                                                    as_str(): "5",
                                                                },
                                                                parsed: 5,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 41,
                                                            end: 42,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 47,
                                                            end: 50,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 52,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 53,
                                                            end: 54,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 60,
                                                                as_str(): "match",
                                                            },
                                                        },
                                                        value: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 61,
                                                                        end: 62,
                                                                        as_str(): "8",
                                                                    },
                                                                    parsed: 8,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        branches: Braces {
                                                            inner: [
                                                                MatchBranch {
                                                                    pattern: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 73,
                                                                                    end: 74,
                                                                                    as_str(): "7",
                                                                                },
                                                                                parsed: 7,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 77,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 80,
                                                                                                    end: 81,
                                                                                                    as_str(): "4",
                                                                                                },
                                                                                                parsed: 4,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 78,
                                                                                end: 83,
                                                                                as_str(): "{ 4 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 83,
                                                                                    end: 84,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 93,
                                                                                    end: 94,
                                                                                    as_str(): "9",
                                                                                },
                                                                                parsed: 9,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 95,
                                                                            end: 97,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 100,
                                                                                                    end: 101,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 103,
                                                                                as_str(): "{ 5 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 104,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 113,
                                                                                    end: 114,
                                                                                    as_str(): "8",
                                                                                },
                                                                                parsed: 8,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 115,
                                                                            end: 117,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 120,
                                                                                                    end: 121,
                                                                                                    as_str(): "6",
                                                                                                },
                                                                                                parsed: 6,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 118,
                                                                                end: 123,
                                                                                as_str(): "{ 6 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 123,
                                                                                    end: 124,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Wildcard {
                                                                        underscore_token: UnderscoreToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 133,
                                                                                end: 134,
                                                                                as_str(): "_",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 135,
                                                                            end: 137,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 140,
                                                                                                    end: 143,
                                                                                                    as_str(): "100",
                                                                                                },
                                                                                                parsed: 100,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 138,
                                                                                end: 145,
                                                                                as_str(): "{ 100 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 146,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 152,
                                                                as_str(): "{\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 152,
                                                            end: 153,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 161,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 162,
                                                                end: 163,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 165,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 171,
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
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 173,
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
                                                                    pattern: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 185,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 186,
                                                                            end: 188,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 191,
                                                                                                    end: 193,
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
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 189,
                                                                                end: 195,
                                                                                as_str(): "{ 42 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 195,
                                                                                    end: 196,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Wildcard {
                                                                        underscore_token: UnderscoreToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 205,
                                                                                end: 206,
                                                                                as_str(): "_",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 207,
                                                                            end: 209,
                                                                            as_str(): "=>",
                                                                        },
                                                                    },
                                                                    kind: Block {
                                                                        block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [],
                                                                                final_expr_opt: Some(
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 212,
                                                                                                    end: 214,
                                                                                                    as_str(): "24",
                                                                                                },
                                                                                                parsed: 24,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                ),
                                                                                start: 210,
                                                                                end: 216,
                                                                                as_str(): "{ 24 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                    ),
                                                                                    start: 216,
                                                                                    end: 217,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 174,
                                                                end: 223,
                                                                as_str(): "{\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                            ),
                                                            start: 223,
                                                            end: 224,
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
                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                        ),
                                                        start: 229,
                                                        end: 234,
                                                        as_str(): "match",
                                                    },
                                                },
                                                value: Literal(
                                                    Int(
                                                        LitInt {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                ),
                                                                start: 235,
                                                                end: 237,
                                                                as_str(): "42",
                                                            },
                                                            parsed: 42,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                                branches: Braces {
                                                    inner: [
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 248,
                                                                            end: 249,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 250,
                                                                    end: 252,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Block {
                                                                block: Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                            ),
                                                                                            start: 255,
                                                                                            end: 257,
                                                                                            as_str(): "24",
                                                                                        },
                                                                                        parsed: 24,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 253,
                                                                        end: 259,
                                                                        as_str(): "{ 24 }",
                                                                    },
                                                                },
                                                                comma_token_opt: Some(
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 260,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 269,
                                                                        end: 272,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c8b9aa10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                    ),
                                                                    start: 273,
                                                                    end: 275,
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
                                                                                                src (ptr): 0x00007fe0c8b9aa10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                                                ),
                                                                                                start: 278,
                                                                                                end: 281,
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
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                        ),
                                                                        start: 276,
                                                                        end: 283,
                                                                        as_str(): "{ foo }",
                                                                    },
                                                                },
                                                                comma_token_opt: Some(
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c8b9aa10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                                            ),
                                                                            start: 283,
                                                                            end: 284,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c8b9aa10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                                        ),
                                                        start: 238,
                                                        end: 290,
                                                        as_str(): "{\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c8b9aa10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRW8NLa8/match_expressions/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 292,
                                        as_str(): "{\n    let x = 5;\n    let a = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let b = match x {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    match 42 {\n        0 => { 24 },\n        foo => { foo },\n    }\n}",
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
