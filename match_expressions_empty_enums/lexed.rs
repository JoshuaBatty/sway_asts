Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0a5ccace0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0a5ccace0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
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
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 14,
                                        end: 21,
                                        as_str(): "MyNever",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 22,
                                        end: 24,
                                        as_str(): "{}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 30,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: None,
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0a5ccace0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                    ),
                                                    start: 31,
                                                    end: 38,
                                                    as_str(): "MyNever",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 45,
                                                            end: 47,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 48,
                                                            end: 56,
                                                            as_str(): "into_any",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: Some(
                                                        GenericParams {
                                                            parameters: AngleBrackets {
                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 56,
                                                                        end: 57,
                                                                        as_str(): "<",
                                                                    },
                                                                },
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                ),
                                                                                start: 57,
                                                                                end: 58,
                                                                                as_str(): "T",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                },
                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 58,
                                                                        end: 59,
                                                                        as_str(): ">",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 60,
                                                                    end: 64,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 65,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 68,
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
                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                ),
                                                                                start: 69,
                                                                                end: 70,
                                                                                as_str(): "T",
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
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 81,
                                                                        end: 86,
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
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 87,
                                                                                    end: 91,
                                                                                    as_str(): "self",
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
                                                                    inner: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 92,
                                                                        end: 94,
                                                                        as_str(): "{}",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a5ccace0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                        ),
                                                        start: 71,
                                                        end: 100,
                                                        as_str(): "{\n        match self {}\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 39,
                                        end: 102,
                                        as_str(): "{\n    fn into_any<T>(self) -> T {\n        match self {}\n    }\n}",
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
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 104,
                                            end: 106,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 107,
                                            end: 121,
                                            as_str(): "result_into_ok",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a5ccace0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                        ),
                                                        start: 121,
                                                        end: 122,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 123,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a5ccace0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                        ),
                                                        start: 123,
                                                        end: 124,
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
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 125,
                                                                    end: 128,
                                                                    as_str(): "res",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                ),
                                                                start: 128,
                                                                end: 129,
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
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 130,
                                                                            end: 136,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: Some(
                                                                        (
                                                                            None,
                                                                            GenericArgs {
                                                                                parameters: AngleBrackets {
                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 136,
                                                                                            end: 137,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                    },
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                Path(
                                                                                                    PathType {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathTypeSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 137,
                                                                                                                    end: 138,
                                                                                                                    as_str(): "T",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                    },
                                                                                                ),
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                        ),
                                                                                                        start: 138,
                                                                                                        end: 139,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                                                ),
                                                                                                                start: 140,
                                                                                                                end: 147,
                                                                                                                as_str(): "MyNever",
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
                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                            ),
                                                                                            start: 147,
                                                                                            end: 148,
                                                                                            as_str(): ">",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 124,
                                            end: 149,
                                            as_str(): "(res: Result<T, MyNever>)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0a5ccace0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                    ),
                                                    start: 150,
                                                    end: 152,
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
                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                ),
                                                                start: 153,
                                                                end: 154,
                                                                as_str(): "T",
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
                                                        src (ptr): 0x00007fe0a5ccace0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                        ),
                                                        start: 161,
                                                        end: 166,
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
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 167,
                                                                    end: 170,
                                                                    as_str(): "res",
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
                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                ),
                                                                                start: 181,
                                                                                end: 187,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 187,
                                                                                    end: 189,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 189,
                                                                                        end: 191,
                                                                                        as_str(): "Ok",
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
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 192,
                                                                                        end: 193,
                                                                                        as_str(): "t",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 191,
                                                                        end: 194,
                                                                        as_str(): "(t)",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 195,
                                                                    end: 197,
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
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 198,
                                                                                    end: 199,
                                                                                    as_str(): "t",
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
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 199,
                                                                        end: 200,
                                                                        as_str(): ",",
                                                                    },
                                                                },
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
                                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                ),
                                                                                start: 375,
                                                                                end: 381,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                    ),
                                                                                    start: 381,
                                                                                    end: 383,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 383,
                                                                                        end: 386,
                                                                                        as_str(): "Err",
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
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 387,
                                                                                        end: 392,
                                                                                        as_str(): "never",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 386,
                                                                        end: 393,
                                                                        as_str(): "(never)",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0a5ccace0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                    ),
                                                                    start: 394,
                                                                    end: 396,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: Match {
                                                                    match_token: MatchToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 397,
                                                                            end: 402,
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
                                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                                        ),
                                                                                        start: 403,
                                                                                        end: 408,
                                                                                        as_str(): "never",
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
                                                                        inner: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0a5ccace0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                            ),
                                                                            start: 409,
                                                                            end: 411,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                },
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0a5ccace0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                        ),
                                                                        start: 411,
                                                                        end: 412,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0a5ccace0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                        ),
                                                        start: 171,
                                                        end: 418,
                                                        as_str(): "{\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 155,
                                        end: 420,
                                        as_str(): "{\n    match res {\n        Result::Ok(t) => t,\n        // This branch can never be taken, and so the\n        // compiler is happy to treat it as evaluating\n        // to whatever type we wish - in this case, `T`.\n        Result::Err(never) => match never {},\n    }\n}",
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
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 422,
                                            end: 424,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 425,
                                            end: 429,
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
                                            src (ptr): 0x00007fe0a5ccace0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                            ),
                                            start: 429,
                                            end: 431,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0a5ccace0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                    ),
                                                    start: 432,
                                                    end: 434,
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
                                                                src (ptr): 0x00007fe0a5ccace0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                                ),
                                                                start: 435,
                                                                end: 438,
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
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0a5ccace0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                                            ),
                                                            start: 445,
                                                            end: 447,
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
                                        src (ptr): 0x00007fe0a5ccace0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkoYEPV/match_expressions_empty_enums/src/main.sw",
                                        ),
                                        start: 439,
                                        end: 449,
                                        as_str(): "{\n    42\n}",
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
