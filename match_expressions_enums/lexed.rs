Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0a9b0a660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0a9b0a660,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0a9b0a660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0a9b0a660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                        ),
                                        start: 14,
                                        end: 15,
                                        as_str(): "X",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 23,
                                                                as_str(): "Y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 23,
                                                                end: 24,
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
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 25,
                                                                            end: 28,
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
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a9b0a660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                        ),
                                                        start: 28,
                                                        end: 29,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: Some(
                                            Annotated {
                                                attribute_list: [],
                                                value: TypeField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 34,
                                                            end: 35,
                                                            as_str(): "Z",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 35,
                                                            end: 36,
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
                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                        ),
                                                                        start: 37,
                                                                        end: 41,
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
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0a9b0a660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 43,
                                        as_str(): "{\n    Y: u64,\n    Z: bool\n}",
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
                                            src (ptr): 0x00007fe0a9b0a660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                            ),
                                            start: 45,
                                            end: 47,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a9b0a660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 52,
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
                                            src (ptr): 0x00007fe0a9b0a660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                            ),
                                            start: 52,
                                            end: 54,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0a9b0a660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                    ),
                                                    start: 55,
                                                    end: 57,
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
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 58,
                                                                end: 61,
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
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 68,
                                                            end: 71,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 72,
                                                                end: 73,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 74,
                                                            end: 75,
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
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 76,
                                                                            end: 77,
                                                                            as_str(): "X",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 77,
                                                                                end: 79,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 79,
                                                                                    end: 80,
                                                                                    as_str(): "Y",
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
                                                                final_value_opt: Some(
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 81,
                                                                                    end: 83,
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
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 80,
                                                                end: 84,
                                                                as_str(): "(42)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 84,
                                                            end: 85,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 93,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 95,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 96,
                                                            end: 97,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Match {
                                                        match_token: MatchToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 103,
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
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 104,
                                                                            end: 105,
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
                                                                    pattern: Constructor {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 116,
                                                                                        end: 117,
                                                                                        as_str(): "X",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
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
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 119,
                                                                                                end: 120,
                                                                                                as_str(): "Y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    Var {
                                                                                        reference: None,
                                                                                        mutable: None,
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 121,
                                                                                                end: 123,
                                                                                                as_str(): "hi",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 120,
                                                                                end: 124,
                                                                                as_str(): "(hi)",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 125,
                                                                            end: 127,
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
                                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 130,
                                                                                                        end: 132,
                                                                                                        as_str(): "hi",
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
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 128,
                                                                                end: 134,
                                                                                as_str(): "{ hi }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 134,
                                                                                    end: 135,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                                MatchBranch {
                                                                    pattern: Constructor {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a9b0a660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 144,
                                                                                        end: 145,
                                                                                        as_str(): "X",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 145,
                                                                                            end: 147,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                ),
                                                                                                start: 147,
                                                                                                end: 148,
                                                                                                as_str(): "Z",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 149,
                                                                                                    end: 154,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 148,
                                                                                end: 155,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 156,
                                                                            end: 158,
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
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 161,
                                                                                                    end: 162,
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
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 164,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 164,
                                                                                    end: 165,
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
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 174,
                                                                                end: 175,
                                                                                as_str(): "_",
                                                                            },
                                                                        },
                                                                    },
                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a9b0a660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                            ),
                                                                            start: 176,
                                                                            end: 178,
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
                                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                                    ),
                                                                                                    start: 181,
                                                                                                    end: 182,
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
                                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                ),
                                                                                start: 179,
                                                                                end: 184,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                        comma_token_opt: Some(
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a9b0a660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 185,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 191,
                                                                as_str(): "{\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a9b0a660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                            ),
                                                            start: 191,
                                                            end: 192,
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
                                                                src (ptr): 0x00007fe0a9b0a660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                                                ),
                                                                start: 202,
                                                                end: 203,
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
                                        src (ptr): 0x00007fe0a9b0a660,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlpmYaT/match_expressions_enums/src/main.sw",
                                        ),
                                        start: 62,
                                        end: 205,
                                        as_str(): "{\n    let a = X::Y(42);\n    let b = match a {\n        X::Y(hi) => { hi },\n        X::Z(false) => { 0 },\n        _ => { 0 },\n    };\n    \n    b\n}",
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
