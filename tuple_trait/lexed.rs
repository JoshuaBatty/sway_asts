Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a1d5dbf10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a1d5dbf10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
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
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
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
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 17,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
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
                                                src (ptr): 0x00007f8a1d5dbf10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 22,
                                                as_str(): "ops",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007f8a1d5dbf10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                ),
                                                start: 22,
                                                end: 24,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Glob {
                                            star_token: StarToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
                                                    as_str(): "*",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 26,
                                        as_str(): ";",
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
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 28,
                                        end: 32,
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
                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                        ),
                                                        start: 33,
                                                        end: 35,
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
                                                src (ptr): 0x00007f8a1d5dbf10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                ),
                                                start: 36,
                                                end: 39,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Tuple(
                                    Parens {
                                        inner: Cons {
                                            head: Path(
                                                PathType {
                                                    root_opt: None,
                                                    prefix: PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 41,
                                                                end: 44,
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
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 44,
                                                    end: 45,
                                                    as_str(): ",",
                                                },
                                            },
                                            tail: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 46,
                                                                            end: 49,
                                                                            as_str(): "u64",
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
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 49,
                                                                end: 50,
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
                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                        ),
                                                                        start: 51,
                                                                        end: 54,
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
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 55,
                                            as_str(): "(u64, u64, u64)",
                                        },
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 62,
                                                            end: 64,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 65,
                                                            end: 67,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                    ),
                                                                    start: 68,
                                                                    end: 72,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 72,
                                                                            end: 73,
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
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 74,
                                                                                            end: 79,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 79,
                                                                                        end: 80,
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
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 81,
                                                                                                    end: 85,
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 86,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                    ),
                                                                    start: 87,
                                                                    end: 89,
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
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 90,
                                                                                end: 94,
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
                                                            LogicalAnd {
                                                                lhs: LogicalAnd {
                                                                    lhs: Equal {
                                                                        lhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                ),
                                                                                                start: 105,
                                                                                                end: 109,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                    ),
                                                                                    start: 109,
                                                                                    end: 110,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 0,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 110,
                                                                                end: 111,
                                                                                as_str(): "0",
                                                                            },
                                                                        },
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 112,
                                                                                end: 114,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                ),
                                                                                                start: 115,
                                                                                                end: 120,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                    ),
                                                                                    start: 120,
                                                                                    end: 121,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 0,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 121,
                                                                                end: 122,
                                                                                as_str(): "0",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 123,
                                                                            end: 125,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                ),
                                                                                                start: 126,
                                                                                                end: 130,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                    ),
                                                                                    start: 130,
                                                                                    end: 131,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 1,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 131,
                                                                                end: 132,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 133,
                                                                                end: 135,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                ),
                                                                                                start: 136,
                                                                                                end: 141,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                    ),
                                                                                    start: 141,
                                                                                    end: 142,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 1,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 142,
                                                                                end: 143,
                                                                                as_str(): "1",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                double_ampersand_token: DoubleAmpersandToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                        ),
                                                                        start: 144,
                                                                        end: 146,
                                                                        as_str(): "&&",
                                                                    },
                                                                },
                                                                rhs: Equal {
                                                                    lhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 147,
                                                                                            end: 151,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 151,
                                                                                end: 152,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 2,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 152,
                                                                            end: 153,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 154,
                                                                            end: 156,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 157,
                                                                                            end: 162,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 162,
                                                                                end: 163,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 2,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 163,
                                                                            end: 164,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                        ),
                                                        start: 95,
                                                        end: 170,
                                                        as_str(): "{\n        self.0 == other.0 && self.1 == other.1 && self.2 == other.2\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 56,
                                        end: 172,
                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1 && self.2 == other.2\n    }\n}",
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
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 174,
                                        end: 178,
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
                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                        ),
                                                        start: 179,
                                                        end: 181,
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
                                                src (ptr): 0x00007f8a1d5dbf10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                ),
                                                start: 182,
                                                end: 185,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Tuple(
                                    Parens {
                                        inner: Cons {
                                            head: Path(
                                                PathType {
                                                    root_opt: None,
                                                    prefix: PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 187,
                                                                end: 190,
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
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 190,
                                                    end: 191,
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
                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 195,
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
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 196,
                                            as_str(): "(u64, u64)",
                                        },
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 203,
                                                            end: 205,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 208,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                    ),
                                                                    start: 209,
                                                                    end: 213,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 213,
                                                                            end: 214,
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
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 215,
                                                                                            end: 220,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 220,
                                                                                        end: 221,
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
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 222,
                                                                                                    end: 226,
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 208,
                                                            end: 227,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                    ),
                                                                    start: 228,
                                                                    end: 230,
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
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 231,
                                                                                end: 235,
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
                                                            LogicalAnd {
                                                                lhs: Equal {
                                                                    lhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 246,
                                                                                            end: 250,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 250,
                                                                                end: 251,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 251,
                                                                            end: 252,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 253,
                                                                            end: 255,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 256,
                                                                                            end: 261,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 261,
                                                                                end: 262,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 262,
                                                                            end: 263,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                },
                                                                double_ampersand_token: DoubleAmpersandToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                        ),
                                                                        start: 264,
                                                                        end: 266,
                                                                        as_str(): "&&",
                                                                    },
                                                                },
                                                                rhs: Equal {
                                                                    lhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 267,
                                                                                            end: 271,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 271,
                                                                                end: 272,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 272,
                                                                            end: 273,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 274,
                                                                            end: 276,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 277,
                                                                                            end: 282,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 282,
                                                                                end: 283,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 283,
                                                                            end: 284,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                        ),
                                                        start: 236,
                                                        end: 290,
                                                        as_str(): "{\n        self.0 == other.0 && self.1 == other.1\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 197,
                                        end: 292,
                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1\n    }\n}",
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
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 294,
                                            end: 296,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 297,
                                            end: 301,
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
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 301,
                                            end: 303,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 304,
                                                    end: 306,
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
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 307,
                                                                end: 311,
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 318,
                                                            end: 321,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 322,
                                                                end: 323,
                                                                as_str(): "t",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 324,
                                                            end: 325,
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
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 327,
                                                                                end: 329,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                        ),
                                                                        start: 329,
                                                                        end: 330,
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
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 331,
                                                                                        end: 333,
                                                                                        as_str(): "43",
                                                                                    },
                                                                                    parsed: 43,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 326,
                                                                end: 334,
                                                                as_str(): "(42, 43)",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 335,
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
                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                        ),
                                                                        start: 340,
                                                                        end: 346,
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
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 347,
                                                                                        end: 348,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 349,
                                                                            end: 351,
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
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 352,
                                                                                        end: 353,
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
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 346,
                                                            end: 354,
                                                            as_str(): "(t == t)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 355,
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
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 361,
                                                            end: 365,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 312,
                                        end: 367,
                                        as_str(): "{\n    let t = (42, 43);\n    assert(t == t);\n\n    true\n}",
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
