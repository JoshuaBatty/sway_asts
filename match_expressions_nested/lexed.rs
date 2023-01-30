Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe09612d790,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe09612d790,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
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
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 14,
                                        end: 17,
                                        as_str(): "Foo",
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
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 24,
                                                                end: 27,
                                                                as_str(): "Bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 27,
                                                                end: 28,
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
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 29,
                                                                            end: 33,
                                                                            as_str(): "Zoom",
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
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 33,
                                                        end: 34,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 36,
                                        as_str(): "{\n    Bar: Zoom,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 38,
                                        end: 42,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 47,
                                        as_str(): "Zoom",
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
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 54,
                                                                end: 57,
                                                                as_str(): "Wow",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 57,
                                                                end: 58,
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
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 59,
                                                                            end: 62,
                                                                            as_str(): "u32",
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
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 62,
                                                        end: 63,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 65,
                                        as_str(): "{\n    Wow: u32,\n}",
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
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 67,
                                            end: 69,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 70,
                                            end: 74,
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
                                            src (ptr): 0x00007fe09612d790,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                            ),
                                            start: 74,
                                            end: 76,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09612d790,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 79,
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
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 80,
                                                                end: 83,
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
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
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
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 94,
                                                                end: 95,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 96,
                                                            end: 97,
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
                                                                            src (ptr): 0x00007fe09612d790,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                            ),
                                                                            start: 98,
                                                                            end: 101,
                                                                            as_str(): "Foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 101,
                                                                                end: 103,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                    ),
                                                                                    start: 103,
                                                                                    end: 106,
                                                                                    as_str(): "Bar",
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
                                                                    FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09612d790,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                            ),
                                                                                            start: 107,
                                                                                            end: 111,
                                                                                            as_str(): "Zoom",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 111,
                                                                                                end: 113,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 113,
                                                                                                    end: 116,
                                                                                                    as_str(): "Wow",
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
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 117,
                                                                                                    end: 120,
                                                                                                    as_str(): "123",
                                                                                                },
                                                                                                parsed: 123,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 116,
                                                                                end: 121,
                                                                                as_str(): "(123)",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09612d790,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 122,
                                                                as_str(): "(Zoom::Wow(123))",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09612d790,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                            ),
                                                            start: 122,
                                                            end: 123,
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
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 128,
                                                        end: 133,
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
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 135,
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
                                                            pattern: Constructor {
                                                                path: PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09612d790,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                ),
                                                                                start: 146,
                                                                                end: 149,
                                                                                as_str(): "Foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
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
                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                        ),
                                                                                        start: 151,
                                                                                        end: 154,
                                                                                        as_str(): "Bar",
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
                                                                            Constructor {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09612d790,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                ),
                                                                                                start: 155,
                                                                                                end: 159,
                                                                                                as_str(): "Zoom",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                    ),
                                                                                                    start: 159,
                                                                                                    end: 161,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 161,
                                                                                                        end: 164,
                                                                                                        as_str(): "Wow",
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
                                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                                        ),
                                                                                                        start: 165,
                                                                                                        end: 166,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09612d790,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                                        ),
                                                                                        start: 164,
                                                                                        end: 167,
                                                                                        as_str(): "(x)",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09612d790,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                        ),
                                                                        start: 154,
                                                                        end: 168,
                                                                        as_str(): "(Zoom::Wow(x))",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09612d790,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                    ),
                                                                    start: 169,
                                                                    end: 171,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09612d790,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
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
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09612d790,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                                        ),
                                                                        start: 173,
                                                                        end: 174,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09612d790,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                                        ),
                                                        start: 136,
                                                        end: 180,
                                                        as_str(): "{\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09612d790,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRGlJm8O/match_expressions_nested/src/main.sw",
                                        ),
                                        start: 84,
                                        end: 182,
                                        as_str(): "{\n    let x = Foo::Bar(Zoom::Wow(123));\n    match x {\n        Foo::Bar(Zoom::Wow(x)) => x,\n    }\n}",
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
