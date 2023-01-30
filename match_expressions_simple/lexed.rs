Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe09ea4f9c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe09ea4f9c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                            src (ptr): 0x00007fe09ea4f9c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 36,
                                                                end: 37,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 52,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 53,
                                                            end: 54,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 78,
                                                                                end: 83,
                                                                                as_str(): "{ 4 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 103,
                                                                                as_str(): "{ 5 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 118,
                                                                                end: 123,
                                                                                as_str(): "{ 6 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 133,
                                                                                end: 134,
                                                                                as_str(): "_",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 138,
                                                                                end: 145,
                                                                                as_str(): "{ 100 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 152,
                                                                as_str(): "{\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 162,
                                                                end: 163,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 165,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 173,
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
                                                        branches: Braces {
                                                            inner: [
                                                                MatchBranch {
                                                                    pattern: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 189,
                                                                                end: 195,
                                                                                as_str(): "{ 42 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 205,
                                                                                end: 206,
                                                                                as_str(): "_",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 210,
                                                                                end: 216,
                                                                                as_str(): "{ 24 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 174,
                                                                end: 223,
                                                                as_str(): "{\n        5 => { 42 },\n        _ => { 24 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 223,
                                                            end: 224,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 229,
                                                            end: 232,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 233,
                                                                end: 234,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 236,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 237,
                                                                end: 242,
                                                                as_str(): "match",
                                                            },
                                                        },
                                                        value: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 243,
                                                                        end: 245,
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
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 256,
                                                                                    end: 257,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 258,
                                                                            end: 260,
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 263,
                                                                                                    end: 265,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 261,
                                                                                end: 267,
                                                                                as_str(): "{ 24 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 267,
                                                                                    end: 268,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 277,
                                                                                end: 280,
                                                                                as_str(): "foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 281,
                                                                            end: 283,
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
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 286,
                                                                                                        end: 289,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 284,
                                                                                end: 291,
                                                                                as_str(): "{ foo }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 291,
                                                                                    end: 292,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 246,
                                                                end: 298,
                                                                as_str(): "{\n        0 => { 24 },\n        foo => { foo },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 298,
                                                            end: 299,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 304,
                                                            end: 307,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 308,
                                                                end: 309,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 311,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Tuple(
                                                                    Parens {
                                                                        inner: Cons {
                                                                            head: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 323,
                                                                                            end: 324,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            comma_token: CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 324,
                                                                                    end: 325,
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 326,
                                                                                                    end: 327,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                                parsed: 2,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 322,
                                                                            end: 328,
                                                                            as_str(): "(1, 2)",
                                                                        },
                                                                    },
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 328,
                                                                        end: 329,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Tuple(
                                                                            Parens {
                                                                                inner: Cons {
                                                                                    head: Tuple(
                                                                                        Parens {
                                                                                            inner: Cons {
                                                                                                head: Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 353,
                                                                                                                end: 354,
                                                                                                                as_str(): "3",
                                                                                                            },
                                                                                                            parsed: 3,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                        ),
                                                                                                        start: 354,
                                                                                                        end: 355,
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
                                                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 356,
                                                                                                                        end: 357,
                                                                                                                        as_str(): "4",
                                                                                                                    },
                                                                                                                    parsed: 4,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 352,
                                                                                                end: 358,
                                                                                                as_str(): "(3, 4)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    comma_token: CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 358,
                                                                                            end: 359,
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
                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                            ),
                                                                                                            start: 372,
                                                                                                            end: 373,
                                                                                                            as_str(): "5",
                                                                                                        },
                                                                                                        parsed: 5,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 338,
                                                                                    end: 383,
                                                                                    as_str(): "(\n            (3, 4),\n            5\n        )",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 312,
                                                                end: 389,
                                                                as_str(): "(\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    )",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 389,
                                                            end: 390,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 395,
                                                            end: 398,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 399,
                                                                end: 400,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 401,
                                                            end: 402,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 403,
                                                                end: 408,
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 409,
                                                                            end: 410,
                                                                            as_str(): "e",
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
                                                                                        Tuple(
                                                                                            Parens {
                                                                                                inner: Punctuated {
                                                                                                    value_separator_pairs: [
                                                                                                        (
                                                                                                            Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 423,
                                                                                                                            end: 424,
                                                                                                                            as_str(): "3",
                                                                                                                        },
                                                                                                                        parsed: 3,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                            CommaToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 424,
                                                                                                                    end: 425,
                                                                                                                    as_str(): ",",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    final_value_opt: Some(
                                                                                                        Wildcard {
                                                                                                            underscore_token: UnderscoreToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 426,
                                                                                                                    end: 427,
                                                                                                                    as_str(): "_",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 422,
                                                                                                    end: 428,
                                                                                                    as_str(): "(3, _)",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 428,
                                                                                                end: 429,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Wildcard {
                                                                                        underscore_token: UnderscoreToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 430,
                                                                                                end: 431,
                                                                                                as_str(): "_",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 421,
                                                                                end: 432,
                                                                                as_str(): "((3, _), _)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 433,
                                                                            end: 435,
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 438,
                                                                                                    end: 440,
                                                                                                    as_str(): "99",
                                                                                                },
                                                                                                parsed: 99,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 436,
                                                                                end: 442,
                                                                                as_str(): "{ 99 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 442,
                                                                                    end: 443,
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
                                                                                        Wildcard {
                                                                                            underscore_token: UnderscoreToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 453,
                                                                                                    end: 454,
                                                                                                    as_str(): "_",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 454,
                                                                                                end: 455,
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
                                                                                                        Wildcard {
                                                                                                            underscore_token: UnderscoreToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 457,
                                                                                                                    end: 458,
                                                                                                                    as_str(): "_",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                ),
                                                                                                                start: 458,
                                                                                                                end: 459,
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
                                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 460,
                                                                                                                    end: 461,
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
                                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                ),
                                                                                                start: 456,
                                                                                                end: 462,
                                                                                                as_str(): "(_, 5)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 452,
                                                                                end: 463,
                                                                                as_str(): "(_, (_, 5))",
                                                                            },
                                                                        },
                                                                    ),
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 464,
                                                                            end: 466,
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 469,
                                                                                                    end: 471,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 467,
                                                                                end: 473,
                                                                                as_str(): "{ 42 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 473,
                                                                                    end: 474,
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
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 483,
                                                                                end: 484,
                                                                                as_str(): "_",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 485,
                                                                            end: 487,
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
                                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                                    ),
                                                                                                    start: 490,
                                                                                                    end: 491,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 488,
                                                                                end: 493,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 493,
                                                                                    end: 494,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                ),
                                                                start: 411,
                                                                end: 500,
                                                                as_str(): "{\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 500,
                                                            end: 501,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 507,
                                                            end: 512,
                                                            as_str(): "match",
                                                        },
                                                    },
                                                    value: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 513,
                                                                    end: 517,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    branches: Braces {
                                                        inner: [
                                                            MatchBranch {
                                                                pattern: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 528,
                                                                                end: 532,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 533,
                                                                        end: 535,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: Tuple(
                                                                        Parens {
                                                                            inner: Nil,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 536,
                                                                                end: 538,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    ),
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 538,
                                                                            end: 539,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 548,
                                                                                end: 553,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 554,
                                                                        end: 556,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: Tuple(
                                                                        Parens {
                                                                            inner: Nil,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 557,
                                                                                end: 559,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    ),
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 559,
                                                                            end: 560,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Var {
                                                                    reference: None,
                                                                    mutable: None,
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 569,
                                                                            end: 572,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 573,
                                                                        end: 575,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: Tuple(
                                                                        Parens {
                                                                            inner: Nil,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 576,
                                                                                end: 578,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    ),
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 578,
                                                                            end: 579,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 518,
                                                            end: 623,
                                                            as_str(): "{\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 629,
                                                            end: 631,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        LogicalAnd {
                                                            lhs: LogicalAnd {
                                                                lhs: LogicalAnd {
                                                                    lhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 632,
                                                                                            end: 633,
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
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 637,
                                                                                        end: 638,
                                                                                        as_str(): "6",
                                                                                    },
                                                                                    parsed: 6,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 639,
                                                                            end: 641,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 642,
                                                                                            end: 643,
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
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 644,
                                                                                end: 646,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 647,
                                                                                        end: 649,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                    parsed: 42,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                                double_ampersand_token: DoubleAmpersandToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 650,
                                                                        end: 652,
                                                                        as_str(): "&&",
                                                                    },
                                                                },
                                                                rhs: Equal {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                        ),
                                                                                        start: 653,
                                                                                        end: 654,
                                                                                        as_str(): "d",
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
                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                            ),
                                                                            start: 655,
                                                                            end: 657,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 658,
                                                                                    end: 660,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            double_ampersand_token: DoubleAmpersandToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 661,
                                                                    end: 663,
                                                                    as_str(): "&&",
                                                                },
                                                            },
                                                            rhs: Equal {
                                                                lhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                    ),
                                                                                    start: 664,
                                                                                    end: 665,
                                                                                    as_str(): "f",
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
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 666,
                                                                        end: 668,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 669,
                                                                                end: 671,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09ea4f9c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                ),
                                                                                start: 682,
                                                                                end: 684,
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
                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                            ),
                                                            start: 672,
                                                            end: 690,
                                                            as_str(): "{\n        42\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09ea4f9c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                    ),
                                                                    start: 691,
                                                                    end: 695,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09ea4f9c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                                            ),
                                                                                            start: 706,
                                                                                            end: 707,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                        parsed: 0,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09ea4f9c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                                                        ),
                                                                        start: 696,
                                                                        end: 713,
                                                                        as_str(): "{\n        0\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09ea4f9c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR2GrK3J/match_expressions_simple/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 715,
                                        as_str(): "{\n    let a = 5;\n    let b = match 8 {\n        7 => { 4 },\n        9 => { 5 },\n        8 => { 6 },\n        _ => { 100 },\n    };\n    let c = match a {\n        5 => { 42 },\n        _ => { 24 },\n    };\n    let d = match 42 {\n        0 => { 24 },\n        foo => { foo },\n    };\n    let e = (\n        (1, 2),\n        (\n            (3, 4),\n            5\n        )\n    );\n    let f = match e {\n        ((3, _), _) => { 99 },\n        (_, (_, 5)) => { 42 },\n        _ => { 0 },\n    };\n\n    match true {\n        true => (),\n        false => (),\n        foo => (), // should give an unreachable warning\n    }\n\n    if b == 6 && c == 42 && d == 42 && f == 42 {\n        42\n    } else {\n        0\n    }\n}",
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
