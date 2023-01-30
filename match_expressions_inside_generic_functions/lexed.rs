Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe09f0b11e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe09f0b11e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
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
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 24,
                                                end: 26,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Glob {
                                            star_token: StarToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 27,
                                                    as_str(): "*",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 28,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 29,
                                        end: 32,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 33,
                                            end: 36,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 36,
                                            end: 38,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 38,
                                                end: 44,
                                                as_str(): "revert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe09f0b11e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                ),
                                                start: 44,
                                                end: 46,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Glob {
                                            star_token: StarToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 46,
                                                    end: 47,
                                                    as_str(): "*",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 47,
                                        end: 48,
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 50,
                                            end: 52,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 53,
                                            end: 64,
                                            as_str(): "third_match",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 64,
                                                        end: 65,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 66,
                                                                as_str(): "A",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 66,
                                                        end: 67,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 73,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 74,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 76,
                                                                            as_str(): "A",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 67,
                                            end: 77,
                                            as_str(): "(value: A)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 78,
                                                    end: 80,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 83,
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
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 88,
                                                        end: 93,
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 94,
                                                                    end: 99,
                                                                    as_str(): "value",
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
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 109,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 110,
                                                                    end: 112,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 113,
                                                                                end: 114,
                                                                                as_str(): "5",
                                                                            },
                                                                            parsed: 5,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U8,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 114,
                                                                                        end: 116,
                                                                                        as_str(): "u8",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 116,
                                                                        end: 117,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 100,
                                                        end: 121,
                                                        as_str(): "{\n    foo => 5u8,\n  }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 84,
                                        end: 123,
                                        as_str(): "{\n  match value {\n    foo => 5u8,\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 125,
                                            end: 127,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 140,
                                            as_str(): "second_match",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 140,
                                                        end: 141,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 141,
                                                                end: 142,
                                                                as_str(): "B",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 142,
                                                        end: 143,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 144,
                                                                    end: 149,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 149,
                                                                end: 150,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 151,
                                                                            end: 152,
                                                                            as_str(): "B",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 143,
                                            end: 153,
                                            as_str(): "(value: B)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 154,
                                                    end: 156,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 161,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 166,
                                                        end: 171,
                                                        as_str(): "match",
                                                    },
                                                },
                                                value: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 183,
                                                                        as_str(): "third_match",
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
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 189,
                                                                                    as_str(): "value",
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 183,
                                                            end: 190,
                                                            as_str(): "(value)",
                                                        },
                                                    },
                                                },
                                                branches: Braces {
                                                    inner: [
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 198,
                                                                            as_str(): "1",
                                                                        },
                                                                        parsed: 1,
                                                                        ty_opt: Some(
                                                                            (
                                                                                U8,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 198,
                                                                                    end: 200,
                                                                                    as_str(): "u8",
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 201,
                                                                    end: 203,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 204,
                                                                                end: 209,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 209,
                                                                        end: 210,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 215,
                                                                            end: 216,
                                                                            as_str(): "2",
                                                                        },
                                                                        parsed: 2,
                                                                        ty_opt: Some(
                                                                            (
                                                                                U8,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 216,
                                                                                    end: 218,
                                                                                    as_str(): "u8",
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 219,
                                                                    end: 221,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 222,
                                                                                end: 227,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 227,
                                                                        end: 228,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 233,
                                                                            end: 234,
                                                                            as_str(): "3",
                                                                        },
                                                                        parsed: 3,
                                                                        ty_opt: Some(
                                                                            (
                                                                                U8,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 234,
                                                                                    end: 236,
                                                                                    as_str(): "u8",
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 237,
                                                                    end: 239,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 240,
                                                                                end: 245,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 245,
                                                                        end: 246,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 251,
                                                                            end: 252,
                                                                            as_str(): "4",
                                                                        },
                                                                        parsed: 4,
                                                                        ty_opt: Some(
                                                                            (
                                                                                U8,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 252,
                                                                                    end: 254,
                                                                                    as_str(): "u8",
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 255,
                                                                    end: 257,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 258,
                                                                                end: 263,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 263,
                                                                        end: 264,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 269,
                                                                            end: 270,
                                                                            as_str(): "5",
                                                                        },
                                                                        parsed: 5,
                                                                        ty_opt: Some(
                                                                            (
                                                                                U8,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 270,
                                                                                    end: 272,
                                                                                    as_str(): "u8",
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 273,
                                                                    end: 275,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 276,
                                                                                end: 280,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 280,
                                                                        end: 281,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        MatchBranch {
                                                            pattern: Wildcard {
                                                                underscore_token: UnderscoreToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 286,
                                                                        end: 287,
                                                                        as_str(): "_",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 288,
                                                                    end: 290,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 291,
                                                                                end: 296,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 296,
                                                                        end: 297,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 191,
                                                        end: 301,
                                                        as_str(): "{\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 162,
                                        end: 303,
                                        as_str(): "{\n  match third_match(value) {\n    1u8 => false,\n    2u8 => false,\n    3u8 => false,\n    4u8 => false,\n    5u8 => true,\n    _ => false,\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 305,
                                            end: 307,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 308,
                                            end: 319,
                                            as_str(): "first_match",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 319,
                                                        end: 320,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 320,
                                                                end: 321,
                                                                as_str(): "C",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 321,
                                                        end: 322,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 323,
                                                                    end: 328,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 328,
                                                                end: 329,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 330,
                                                                            end: 331,
                                                                            as_str(): "C",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 322,
                                            end: 332,
                                            as_str(): "(value: C)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 333,
                                                    end: 335,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 336,
                                                                end: 339,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 344,
                                                        end: 349,
                                                        as_str(): "match",
                                                    },
                                                },
                                                value: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 350,
                                                                        end: 362,
                                                                        as_str(): "second_match",
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
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 363,
                                                                                    end: 368,
                                                                                    as_str(): "value",
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 362,
                                                            end: 369,
                                                            as_str(): "(value)",
                                                        },
                                                    },
                                                },
                                                branches: Braces {
                                                    inner: [
                                                        MatchBranch {
                                                            pattern: Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 381,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 382,
                                                                    end: 384,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 385,
                                                                                end: 386,
                                                                                as_str(): "2",
                                                                            },
                                                                            parsed: 2,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 386,
                                                                                        end: 389,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 389,
                                                                        end: 390,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 395,
                                                                            end: 399,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 400,
                                                                    end: 402,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 403,
                                                                                end: 404,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 404,
                                                                                        end: 407,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 407,
                                                                        end: 408,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 370,
                                                        end: 412,
                                                        as_str(): "{\n    false => 2u64,\n    true => 3u64,\n  }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 340,
                                        end: 414,
                                        as_str(): "{\n  match second_match(value) {\n    false => 2u64,\n    true => 3u64,\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 416,
                                            end: 418,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 419,
                                            end: 427,
                                            as_str(): "third_if",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 427,
                                                        end: 428,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 428,
                                                                end: 429,
                                                                as_str(): "D",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 429,
                                                        end: 430,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 431,
                                                                    end: 436,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 436,
                                                                end: 437,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 438,
                                                                            end: 439,
                                                                            as_str(): "D",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 430,
                                            end: 440,
                                            as_str(): "(value: D)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 441,
                                                    end: 443,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 444,
                                                                end: 446,
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
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 451,
                                                            end: 453,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Literal(
                                                            Bool(
                                                                LitBool {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 454,
                                                                        end: 458,
                                                                        as_str(): "true",
                                                                    },
                                                                    kind: True,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 465,
                                                                                end: 466,
                                                                                as_str(): "5",
                                                                            },
                                                                            parsed: 5,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U8,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 466,
                                                                                        end: 468,
                                                                                        as_str(): "u8",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 459,
                                                            end: 472,
                                                            as_str(): "{\n    5u8\n  }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 473,
                                                                    end: 477,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
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
                                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                        ),
                                                                                                        start: 484,
                                                                                                        end: 490,
                                                                                                        as_str(): "revert",
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
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                                ),
                                                                                                                start: 491,
                                                                                                                end: 492,
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
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 490,
                                                                                            end: 493,
                                                                                            as_str(): "(0)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                semicolon_token_opt: Some(
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 493,
                                                                                            end: 494,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                        final_expr_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 478,
                                                                        end: 498,
                                                                        as_str(): "{\n    revert(0);\n  }",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 447,
                                        end: 500,
                                        as_str(): "{\n  if true {\n    5u8\n  } else {\n    revert(0);\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 502,
                                            end: 504,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 505,
                                            end: 514,
                                            as_str(): "second_if",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 514,
                                                        end: 515,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 515,
                                                                end: 516,
                                                                as_str(): "E",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 516,
                                                        end: 517,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 518,
                                                                    end: 523,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 523,
                                                                end: 524,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 525,
                                                                            end: 526,
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 517,
                                            end: 527,
                                            as_str(): "(value: E)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 528,
                                                    end: 530,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 531,
                                                                end: 535,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 540,
                                                            end: 543,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 544,
                                                                end: 549,
                                                                as_str(): "third",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 550,
                                                            end: 551,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 552,
                                                                            end: 560,
                                                                            as_str(): "third_if",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 561,
                                                                                        end: 566,
                                                                                        as_str(): "value",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 560,
                                                                end: 567,
                                                                as_str(): "(value)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 567,
                                                            end: 568,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 571,
                                                            end: 573,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        LogicalOr {
                                                            lhs: LogicalOr {
                                                                lhs: LogicalOr {
                                                                    lhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 574,
                                                                                            end: 579,
                                                                                            as_str(): "third",
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 580,
                                                                                end: 582,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 583,
                                                                                        end: 584,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U8,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 584,
                                                                                                end: 586,
                                                                                                as_str(): "u8",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    double_pipe_token: DoublePipeToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 587,
                                                                            end: 589,
                                                                            as_str(): "||",
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
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 590,
                                                                                            end: 595,
                                                                                            as_str(): "third",
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 596,
                                                                                end: 598,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 599,
                                                                                        end: 600,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U8,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 600,
                                                                                                end: 602,
                                                                                                as_str(): "u8",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                                double_pipe_token: DoublePipeToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 603,
                                                                        end: 605,
                                                                        as_str(): "||",
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
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 606,
                                                                                        end: 611,
                                                                                        as_str(): "third",
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 612,
                                                                            end: 614,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 615,
                                                                                    end: 616,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 616,
                                                                                            end: 618,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            double_pipe_token: DoublePipeToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 619,
                                                                    end: 621,
                                                                    as_str(): "||",
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 622,
                                                                                    end: 627,
                                                                                    as_str(): "third",
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 628,
                                                                        end: 630,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 631,
                                                                                end: 632,
                                                                                as_str(): "4",
                                                                            },
                                                                            parsed: 4,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U8,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 632,
                                                                                        end: 634,
                                                                                        as_str(): "u8",
                                                                                    },
                                                                                ),
                                                                            ),
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
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 641,
                                                                                end: 646,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 635,
                                                            end: 650,
                                                            as_str(): "{\n    false\n  }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 651,
                                                                    end: 655,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Continue(
                                                                IfExpr {
                                                                    if_token: IfToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 658,
                                                                            as_str(): "if",
                                                                        },
                                                                    },
                                                                    condition: Expr(
                                                                        Equal {
                                                                            lhs: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 659,
                                                                                                end: 664,
                                                                                                as_str(): "third",
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
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 665,
                                                                                    end: 667,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 668,
                                                                                            end: 669,
                                                                                            as_str(): "5",
                                                                                        },
                                                                                        parsed: 5,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U8,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 669,
                                                                                                    end: 671,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                    then_block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: Some(
                                                                                Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                ),
                                                                                                start: 678,
                                                                                                end: 682,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 672,
                                                                            end: 686,
                                                                            as_str(): "{\n    true\n  }",
                                                                        },
                                                                    },
                                                                    else_opt: Some(
                                                                        (
                                                                            ElseToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 687,
                                                                                    end: 691,
                                                                                    as_str(): "else",
                                                                                },
                                                                            },
                                                                            Break(
                                                                                Braces {
                                                                                    inner: CodeBlockContents {
                                                                                        statements: [],
                                                                                        final_expr_opt: Some(
                                                                                            Literal(
                                                                                                Bool(
                                                                                                    LitBool {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                            ),
                                                                                                            start: 698,
                                                                                                            end: 703,
                                                                                                            as_str(): "false",
                                                                                                        },
                                                                                                        kind: False,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 692,
                                                                                        end: 707,
                                                                                        as_str(): "{\n    false\n  }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 536,
                                        end: 709,
                                        as_str(): "{\n  let third = third_if(value);\n  if third == 1u8 || third == 2u8 || third == 3u8 || third == 4u8 {\n    false\n  } else if third == 5u8 {\n    true\n  } else {\n    false\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 711,
                                            end: 713,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 714,
                                            end: 722,
                                            as_str(): "first_if",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 722,
                                                        end: 723,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 723,
                                                                end: 724,
                                                                as_str(): "F",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 724,
                                                        end: 725,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 726,
                                                                    end: 731,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 731,
                                                                end: 732,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 733,
                                                                            end: 734,
                                                                            as_str(): "F",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 725,
                                            end: 735,
                                            as_str(): "(value: F)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 736,
                                                    end: 738,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 739,
                                                                end: 742,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 747,
                                                            end: 750,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 751,
                                                                end: 757,
                                                                as_str(): "second",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 759,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 760,
                                                                            end: 769,
                                                                            as_str(): "second_if",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 770,
                                                                                        end: 775,
                                                                                        as_str(): "value",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 769,
                                                                end: 776,
                                                                as_str(): "(value)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 776,
                                                            end: 777,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 780,
                                                            end: 782,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Equal {
                                                            lhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 783,
                                                                                end: 789,
                                                                                as_str(): "second",
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 790,
                                                                    end: 792,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            rhs: Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 793,
                                                                            end: 798,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 805,
                                                                                end: 806,
                                                                                as_str(): "2",
                                                                            },
                                                                            parsed: 2,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 806,
                                                                                        end: 809,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 799,
                                                            end: 813,
                                                            as_str(): "{\n    2u64\n  }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 814,
                                                                    end: 818,
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
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 825,
                                                                                            end: 826,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                        parsed: 3,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U64,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 826,
                                                                                                    end: 829,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 819,
                                                                        end: 833,
                                                                        as_str(): "{\n    3u64\n  }",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 743,
                                        end: 835,
                                        as_str(): "{\n  let second = second_if(value);\n  if second == false {\n    2u64\n  } else {\n    3u64\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 837,
                                            end: 839,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 840,
                                            end: 853,
                                            as_str(): "double_double",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 853,
                                                        end: 854,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 854,
                                                                    end: 855,
                                                                    as_str(): "Y",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 855,
                                                                    end: 856,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 857,
                                                                end: 858,
                                                                as_str(): "Z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 858,
                                                        end: 859,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 860,
                                                                        end: 865,
                                                                        as_str(): "first",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 865,
                                                                    end: 866,
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 867,
                                                                                end: 868,
                                                                                as_str(): "Y",
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 868,
                                                                end: 869,
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 870,
                                                                    end: 876,
                                                                    as_str(): "second",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 876,
                                                                end: 877,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 878,
                                                                            end: 879,
                                                                            as_str(): "Z",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 859,
                                            end: 880,
                                            as_str(): "(first: Y, second: Z)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 881,
                                                    end: 883,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 884,
                                                                end: 885,
                                                                as_str(): "Z",
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 890,
                                                                end: 896,
                                                                as_str(): "second",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 886,
                                        end: 898,
                                        as_str(): "{\n  second\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 900,
                                            end: 902,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 903,
                                            end: 909,
                                            as_str(): "double",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 909,
                                                        end: 910,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 910,
                                                                end: 911,
                                                                as_str(): "Y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 911,
                                                        end: 912,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 913,
                                                                    end: 923,
                                                                    as_str(): "the_second",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 923,
                                                                end: 924,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 925,
                                                                            end: 926,
                                                                            as_str(): "Y",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 912,
                                            end: 927,
                                            as_str(): "(the_second: Y)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 928,
                                                    end: 930,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 931,
                                                                end: 932,
                                                                as_str(): "Y",
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 937,
                                                                    end: 950,
                                                                    as_str(): "double_double",
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 951,
                                                                                end: 956,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 956,
                                                                        end: 957,
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
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 958,
                                                                                end: 968,
                                                                                as_str(): "the_second",
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
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 950,
                                                        end: 969,
                                                        as_str(): "(false, the_second)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 933,
                                        end: 971,
                                        as_str(): "{\n  double_double(false, the_second)\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 973,
                                            end: 975,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 976,
                                            end: 989,
                                            as_str(): "generic_match",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 989,
                                                        end: 990,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 990,
                                                                end: 991,
                                                                as_str(): "G",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 991,
                                                        end: 992,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 993,
                                                                    end: 998,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 998,
                                                                end: 999,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1000,
                                                                            end: 1001,
                                                                            as_str(): "G",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 992,
                                            end: 1002,
                                            as_str(): "(value: G)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1003,
                                                    end: 1005,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1006,
                                                                end: 1009,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1014,
                                                        end: 1019,
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
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1020,
                                                                    end: 1025,
                                                                    as_str(): "value",
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
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1032,
                                                                        end: 1035,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1036,
                                                                    end: 1038,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 1039,
                                                                                end: 1040,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1040,
                                                                                        end: 1043,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1043,
                                                                        end: 1044,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1026,
                                                        end: 1048,
                                                        as_str(): "{\n    foo => 3u64,\n  }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1010,
                                        end: 1050,
                                        as_str(): "{\n  match value {\n    foo => 3u64,\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1052,
                                            end: 1054,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1055,
                                            end: 1065,
                                            as_str(): "generic_if",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1065,
                                                        end: 1066,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1066,
                                                                end: 1067,
                                                                as_str(): "H",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe09f0b11e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                        ),
                                                        start: 1067,
                                                        end: 1068,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1069,
                                                                    end: 1074,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1074,
                                                                end: 1075,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1076,
                                                                            end: 1077,
                                                                            as_str(): "H",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1068,
                                            end: 1078,
                                            as_str(): "(value: H)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1079,
                                                    end: 1081,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1082,
                                                                end: 1085,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1090,
                                                            end: 1092,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Literal(
                                                            Bool(
                                                                LitBool {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1093,
                                                                        end: 1097,
                                                                        as_str(): "true",
                                                                    },
                                                                    kind: True,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                ),
                                                                                start: 1104,
                                                                                end: 1105,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U64,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1105,
                                                                                        end: 1108,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1098,
                                                            end: 1112,
                                                            as_str(): "{\n    3u64\n  }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                    ),
                                                                    start: 1113,
                                                                    end: 1117,
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
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 1124,
                                                                                            end: 1125,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U64,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1125,
                                                                                                    end: 1128,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1118,
                                                                        end: 1132,
                                                                        as_str(): "{\n    1u64\n  }",
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1086,
                                        end: 1134,
                                        as_str(): "{\n  if true {\n    3u64\n  } else {\n    1u64\n  }\n}",
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1136,
                                            end: 1138,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1139,
                                            end: 1143,
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
                                            src (ptr): 0x00007fe09f0b11e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                            ),
                                            start: 1143,
                                            end: 1145,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe09f0b11e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                    ),
                                                    start: 1146,
                                                    end: 1148,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1149,
                                                                end: 1152,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1157,
                                                            end: 1160,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1161,
                                                                end: 1162,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1163,
                                                            end: 1164,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1165,
                                                                            end: 1176,
                                                                            as_str(): "first_match",
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
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1177,
                                                                                    end: 1181,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1176,
                                                                end: 1182,
                                                                as_str(): "(true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1182,
                                                            end: 1183,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1186,
                                                                        end: 1192,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1193,
                                                                                        end: 1194,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1195,
                                                                            end: 1197,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1198,
                                                                                    end: 1199,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1192,
                                                            end: 1200,
                                                            as_str(): "(a == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1200,
                                                            end: 1201,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1205,
                                                            end: 1208,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1209,
                                                                end: 1210,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1211,
                                                            end: 1212,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1213,
                                                                            end: 1224,
                                                                            as_str(): "first_match",
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1225,
                                                                                    end: 1226,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 1226,
                                                                                            end: 1228,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1224,
                                                                end: 1229,
                                                                as_str(): "(1u8)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1229,
                                                            end: 1230,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1233,
                                                                        end: 1239,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1240,
                                                                                        end: 1241,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1242,
                                                                            end: 1244,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1245,
                                                                                    end: 1246,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1239,
                                                            end: 1247,
                                                            as_str(): "(b == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1247,
                                                            end: 1248,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1252,
                                                            end: 1255,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1256,
                                                                end: 1257,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1258,
                                                            end: 1259,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1260,
                                                                            end: 1268,
                                                                            as_str(): "first_if",
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
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1269,
                                                                                    end: 1273,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1268,
                                                                end: 1274,
                                                                as_str(): "(true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1274,
                                                            end: 1275,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1278,
                                                                        end: 1284,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1285,
                                                                                        end: 1286,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1287,
                                                                            end: 1289,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1290,
                                                                                    end: 1291,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1284,
                                                            end: 1292,
                                                            as_str(): "(c == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1292,
                                                            end: 1293,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1297,
                                                            end: 1300,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1301,
                                                                end: 1302,
                                                                as_str(): "d",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1303,
                                                            end: 1304,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1305,
                                                                            end: 1313,
                                                                            as_str(): "first_if",
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1314,
                                                                                    end: 1315,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 1315,
                                                                                            end: 1317,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1313,
                                                                end: 1318,
                                                                as_str(): "(1u8)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1318,
                                                            end: 1319,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1322,
                                                                        end: 1328,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1329,
                                                                                        end: 1330,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1331,
                                                                            end: 1333,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1334,
                                                                                    end: 1335,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1328,
                                                            end: 1336,
                                                            as_str(): "(d == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1336,
                                                            end: 1337,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1341,
                                                            end: 1344,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1345,
                                                                end: 1346,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1347,
                                                            end: 1348,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1349,
                                                                            end: 1362,
                                                                            as_str(): "generic_match",
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1363,
                                                                                    end: 1364,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1362,
                                                                end: 1365,
                                                                as_str(): "(6)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1365,
                                                            end: 1366,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1369,
                                                                        end: 1375,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1376,
                                                                                        end: 1377,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1378,
                                                                            end: 1380,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1381,
                                                                                    end: 1382,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1375,
                                                            end: 1383,
                                                            as_str(): "(e == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1383,
                                                            end: 1384,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1388,
                                                            end: 1391,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1392,
                                                                end: 1393,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1394,
                                                            end: 1395,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1396,
                                                                            end: 1409,
                                                                            as_str(): "generic_match",
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
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1410,
                                                                                    end: 1415,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1409,
                                                                end: 1416,
                                                                as_str(): "(false)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1416,
                                                            end: 1417,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1420,
                                                                        end: 1426,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1427,
                                                                                        end: 1428,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1429,
                                                                            end: 1431,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1432,
                                                                                    end: 1433,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1426,
                                                            end: 1434,
                                                            as_str(): "(f == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1434,
                                                            end: 1435,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1439,
                                                            end: 1442,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1443,
                                                                end: 1444,
                                                                as_str(): "g",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1445,
                                                            end: 1446,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1447,
                                                                            end: 1457,
                                                                            as_str(): "generic_if",
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1458,
                                                                                    end: 1459,
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
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1457,
                                                                end: 1460,
                                                                as_str(): "(6)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1460,
                                                            end: 1461,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1464,
                                                                        end: 1470,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1471,
                                                                                        end: 1472,
                                                                                        as_str(): "g",
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1473,
                                                                            end: 1475,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1476,
                                                                                    end: 1477,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1470,
                                                            end: 1478,
                                                            as_str(): "(g == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1478,
                                                            end: 1479,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1483,
                                                            end: 1486,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1487,
                                                                end: 1488,
                                                                as_str(): "h",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1489,
                                                            end: 1490,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1491,
                                                                            end: 1501,
                                                                            as_str(): "generic_if",
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
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1502,
                                                                                    end: 1507,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1501,
                                                                end: 1508,
                                                                as_str(): "(false)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1508,
                                                            end: 1509,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1512,
                                                                        end: 1518,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1519,
                                                                                        end: 1520,
                                                                                        as_str(): "h",
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1521,
                                                                            end: 1523,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1524,
                                                                                    end: 1525,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1518,
                                                            end: 1526,
                                                            as_str(): "(h == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1526,
                                                            end: 1527,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1531,
                                                            end: 1534,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1535,
                                                                end: 1536,
                                                                as_str(): "i",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1537,
                                                            end: 1538,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1539,
                                                                            end: 1545,
                                                                            as_str(): "double",
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1546,
                                                                                    end: 1548,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 1548,
                                                                                            end: 1551,
                                                                                            as_str(): "u32",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1545,
                                                                end: 1552,
                                                                as_str(): "(10u32)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1552,
                                                            end: 1553,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1556,
                                                                        end: 1562,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                        ),
                                                                                        start: 1563,
                                                                                        end: 1564,
                                                                                        as_str(): "i",
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1565,
                                                                            end: 1567,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1568,
                                                                                    end: 1570,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U32,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                            ),
                                                                                            start: 1570,
                                                                                            end: 1573,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1562,
                                                            end: 1574,
                                                            as_str(): "(i == 10u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1574,
                                                            end: 1575,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1579,
                                                            end: 1582,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1583,
                                                                end: 1584,
                                                                as_str(): "j",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1585,
                                                            end: 1586,
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
                                                                            src (ptr): 0x00007fe09f0b11e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                            ),
                                                                            start: 1587,
                                                                            end: 1593,
                                                                            as_str(): "double",
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
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1594,
                                                                                    end: 1598,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe09f0b11e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                ),
                                                                start: 1593,
                                                                end: 1599,
                                                                as_str(): "(true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1599,
                                                            end: 1600,
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
                                                                        src (ptr): 0x00007fe09f0b11e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                        ),
                                                                        start: 1603,
                                                                        end: 1609,
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
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe09f0b11e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                                                    ),
                                                                                    start: 1610,
                                                                                    end: 1611,
                                                                                    as_str(): "j",
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1609,
                                                            end: 1612,
                                                            as_str(): "(j)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1612,
                                                            end: 1613,
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
                                                            src (ptr): 0x00007fe09f0b11e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                                            ),
                                                            start: 1617,
                                                            end: 1618,
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
                                        src (ptr): 0x00007fe09f0b11e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDaXUnN/match_expressions_inside_generic_functions/src/main.sw",
                                        ),
                                        start: 1153,
                                        end: 1620,
                                        as_str(): "{\n  let a = first_match(true);\n  assert(a == 3);\n\n  let b = first_match(1u8);\n  assert(b == 3);\n\n  let c = first_if(true);\n  assert(c == 3);\n\n  let d = first_if(1u8);\n  assert(d == 3);\n\n  let e = generic_match(6);\n  assert(e == 3);\n\n  let f = generic_match(false);\n  assert(f == 3);\n\n  let g = generic_if(6);\n  assert(g == 3);\n\n  let h = generic_if(false);\n  assert(h == 3);\n\n  let i = double(10u32);\n  assert(i == 10u32);\n\n  let j = double(true);\n  assert(j);\n\n  1\n}",
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
