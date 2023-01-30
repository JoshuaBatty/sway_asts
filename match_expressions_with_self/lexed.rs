Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe08b199700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe08b199700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
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
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
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
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 17,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 17,
                                            end: 19,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe08b199700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 22,
                                                as_str(): "ops",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe08b199700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                ),
                                                start: 22,
                                                end: 24,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 26,
                                                    as_str(): "Eq",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 27,
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
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 28,
                                        end: 31,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 32,
                                            end: 35,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 35,
                                            end: 37,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe08b199700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                ),
                                                start: 37,
                                                end: 43,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe08b199700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                ),
                                                start: 43,
                                                end: 45,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Glob {
                                            star_token: StarToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 46,
                                                    as_str(): "*",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 46,
                                        end: 47,
                                        as_str(): ";",
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
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 49,
                                        end: 53,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 54,
                                        end: 65,
                                        as_str(): "Initialized",
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
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 72,
                                                                end: 76,
                                                                as_str(): "True",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 77,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 78,
                                                                    end: 80,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe08b199700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                        ),
                                                        start: 80,
                                                        end: 81,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 86,
                                                                end: 91,
                                                                as_str(): "False",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 91,
                                                                end: 92,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 93,
                                                                    end: 95,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe08b199700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                        ),
                                                        start: 95,
                                                        end: 96,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 66,
                                        end: 98,
                                        as_str(): "{\n    True: (),\n    False: (),\n}",
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
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 100,
                                        end: 104,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: Some(
                                    (
                                        PathType {
                                            root_opt: None,
                                            prefix: PathTypeSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe08b199700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                        ),
                                                        start: 105,
                                                        end: 107,
                                                        as_str(): "Eq",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe08b199700,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                ),
                                                start: 108,
                                                end: 111,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 112,
                                                    end: 123,
                                                    as_str(): "Initialized",
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
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 132,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 133,
                                                            end: 135,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 136,
                                                                    end: 140,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 141,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
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
                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                            ),
                                                                                            start: 142,
                                                                                            end: 147,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 147,
                                                                                        end: 148,
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
                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                    ),
                                                                                                    start: 149,
                                                                                                    end: 153,
                                                                                                    as_str(): "Self",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 154,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b199700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                    ),
                                                                    start: 155,
                                                                    end: 157,
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
                                                                                src (ptr): 0x00007fe08b199700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                ),
                                                                                start: 158,
                                                                                end: 162,
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
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 173,
                                                                        end: 178,
                                                                        as_str(): "match",
                                                                    },
                                                                },
                                                                value: Tuple(
                                                                    Parens {
                                                                        inner: Cons {
                                                                            head: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 180,
                                                                                                end: 184,
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
                                                                            comma_token: CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 184,
                                                                                    end: 185,
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
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 186,
                                                                                                        end: 191,
                                                                                                        as_str(): "other",
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
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 179,
                                                                            end: 192,
                                                                            as_str(): "(self, other)",
                                                                        },
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
                                                                                                Constant(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 208,
                                                                                                                    end: 219,
                                                                                                                    as_str(): "Initialized",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [
                                                                                                            (
                                                                                                                DoubleColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 219,
                                                                                                                        end: 221,
                                                                                                                        as_str(): "::",
                                                                                                                    },
                                                                                                                },
                                                                                                                PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 221,
                                                                                                                            end: 225,
                                                                                                                            as_str(): "True",
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
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 225,
                                                                                                        end: 226,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            Constant(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                ),
                                                                                                                start: 227,
                                                                                                                end: 238,
                                                                                                                as_str(): "Initialized",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [
                                                                                                        (
                                                                                                            DoubleColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 238,
                                                                                                                    end: 240,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 240,
                                                                                                                        end: 244,
                                                                                                                        as_str(): "True",
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
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 207,
                                                                                        end: 245,
                                                                                        as_str(): "(Initialized::True, Initialized::True)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 246,
                                                                                    end: 248,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 249,
                                                                                                end: 253,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 253,
                                                                                        end: 254,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            pattern: Tuple(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                Constant(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 268,
                                                                                                                    end: 279,
                                                                                                                    as_str(): "Initialized",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [
                                                                                                            (
                                                                                                                DoubleColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 279,
                                                                                                                        end: 281,
                                                                                                                        as_str(): "::",
                                                                                                                    },
                                                                                                                },
                                                                                                                PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08b199700,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 281,
                                                                                                                            end: 286,
                                                                                                                            as_str(): "False",
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
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                        ),
                                                                                                        start: 286,
                                                                                                        end: 287,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            Constant(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                ),
                                                                                                                start: 288,
                                                                                                                end: 299,
                                                                                                                as_str(): "Initialized",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [
                                                                                                        (
                                                                                                            DoubleColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 299,
                                                                                                                    end: 301,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 301,
                                                                                                                        end: 306,
                                                                                                                        as_str(): "False",
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
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 307,
                                                                                        as_str(): "(Initialized::False, Initialized::False)",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 308,
                                                                                    end: 310,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 311,
                                                                                                end: 315,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 315,
                                                                                        end: 316,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            pattern: Wildcard {
                                                                                underscore_token: UnderscoreToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 329,
                                                                                        end: 330,
                                                                                        as_str(): "_",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 331,
                                                                                    end: 333,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08b199700,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                                ),
                                                                                                start: 334,
                                                                                                end: 339,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                            kind: False,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 339,
                                                                                        end: 340,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 193,
                                                                        end: 350,
                                                                        as_str(): "{\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe08b199700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                        ),
                                                        start: 163,
                                                        end: 356,
                                                        as_str(): "{\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 124,
                                        end: 358,
                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        match (self, other) {\n            (Initialized::True, Initialized::True) => true,\n            (Initialized::False, Initialized::False) => true,\n            _ => false,\n        }\n    }\n}",
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
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 360,
                                            end: 362,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 363,
                                            end: 367,
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
                                            src (ptr): 0x00007fe08b199700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                            ),
                                            start: 367,
                                            end: 369,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe08b199700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                    ),
                                                    start: 370,
                                                    end: 372,
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
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 373,
                                                                end: 376,
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
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 383,
                                                            end: 386,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 387,
                                                                end: 388,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 389,
                                                            end: 390,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 402,
                                                                        as_str(): "Initialized",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 402,
                                                                            end: 404,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b199700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                ),
                                                                                start: 404,
                                                                                end: 408,
                                                                                as_str(): "True",
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 408,
                                                            end: 409,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 414,
                                                            end: 417,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 418,
                                                                end: 419,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 420,
                                                            end: 421,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 422,
                                                                        end: 433,
                                                                        as_str(): "Initialized",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 433,
                                                                            end: 435,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b199700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                ),
                                                                                start: 435,
                                                                                end: 440,
                                                                                as_str(): "False",
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 440,
                                                            end: 441,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 446,
                                                            end: 449,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 450,
                                                                end: 451,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 452,
                                                            end: 453,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Equal {
                                                        lhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 454,
                                                                            end: 455,
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
                                                                src (ptr): 0x00007fe08b199700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                ),
                                                                start: 456,
                                                                end: 458,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                        rhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 459,
                                                                            end: 460,
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
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 460,
                                                            end: 461,
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
                                                                        src (ptr): 0x00007fe08b199700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                        ),
                                                                        start: 466,
                                                                        end: 472,
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
                                                                                        src (ptr): 0x00007fe08b199700,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                        ),
                                                                                        start: 473,
                                                                                        end: 474,
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
                                                                            src (ptr): 0x00007fe08b199700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                            ),
                                                                            start: 475,
                                                                            end: 477,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b199700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                                                    ),
                                                                                    start: 478,
                                                                                    end: 483,
                                                                                    as_str(): "false",
                                                                                },
                                                                                kind: False,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 472,
                                                            end: 484,
                                                            as_str(): "(c == false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 484,
                                                            end: 485,
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
                                                            src (ptr): 0x00007fe08b199700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                                            ),
                                                            start: 491,
                                                            end: 492,
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
                                        src (ptr): 0x00007fe08b199700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmbduqM/match_expressions_with_self/src/main.sw",
                                        ),
                                        start: 377,
                                        end: 494,
                                        as_str(): "{\n    let a = Initialized::True;\n    let b = Initialized::False;\n    let c = a == b;\n    assert(c == false);\n\n    1\n}",
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
