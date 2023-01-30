Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0bc9cf310,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0bc9cf310,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
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
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
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
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 17,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
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
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 22,
                                                as_str(): "ops",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 22,
                                                end: 24,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Group {
                                            imports: Braces {
                                                inner: Punctuated {
                                                    value_separator_pairs: [
                                                        (
                                                            Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                        ),
                                                                        start: 25,
                                                                        end: 27,
                                                                        as_str(): "Eq",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            CommaToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 27,
                                                                    end: 28,
                                                                    as_str(): ",",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    final_value_opt: Some(
                                                        Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 29,
                                                                    end: 32,
                                                                    as_str(): "Ord",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    ),
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe0bc9cf310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 33,
                                                    as_str(): "{Eq, Ord}",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 34,
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
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 36,
                                        end: 40,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 42,
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
                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                ),
                                                                start: 49,
                                                                end: 50,
                                                                as_str(): "Y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                ),
                                                                start: 50,
                                                                end: 51,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 52,
                                                                    end: 54,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 54,
                                                        end: 55,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 57,
                                        as_str(): "{\n    Y: (),\n}",
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
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 59,
                                        end: 63,
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
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 64,
                                                        end: 66,
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
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 67,
                                                end: 70,
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
                                                    src (ptr): 0x00007fe0bc9cf310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                    ),
                                                    start: 71,
                                                    end: 72,
                                                    as_str(): "X",
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
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 79,
                                                            end: 81,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 82,
                                                            end: 84,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 85,
                                                                    end: 89,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 89,
                                                                            end: 90,
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
                                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                            ),
                                                                                            start: 91,
                                                                                            end: 96,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                        ),
                                                                                        start: 96,
                                                                                        end: 97,
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
                                                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                                    ),
                                                                                                    start: 98,
                                                                                                    end: 102,
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
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 84,
                                                            end: 103,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 106,
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
                                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                ),
                                                                                start: 107,
                                                                                end: 111,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 122,
                                                                            end: 126,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 112,
                                                        end: 132,
                                                        as_str(): "{\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 73,
                                        end: 134,
                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        true\n    }\n}",
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
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 136,
                                        end: 140,
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
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 141,
                                                        end: 144,
                                                        as_str(): "Ord",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0bc9cf310,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                ),
                                                start: 145,
                                                end: 148,
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
                                                    src (ptr): 0x00007fe0bc9cf310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                    ),
                                                    start: 149,
                                                    end: 150,
                                                    as_str(): "X",
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
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 159,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 160,
                                                            end: 162,
                                                            as_str(): "lt",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 163,
                                                                    end: 167,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 167,
                                                                            end: 168,
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
                                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                            ),
                                                                                            start: 169,
                                                                                            end: 174,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                        ),
                                                                                        start: 174,
                                                                                        end: 175,
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
                                                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                                    ),
                                                                                                    start: 176,
                                                                                                    end: 180,
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
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 181,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 182,
                                                                    end: 184,
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
                                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                ),
                                                                                start: 185,
                                                                                end: 189,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 200,
                                                                            end: 205,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 190,
                                                        end: 211,
                                                        as_str(): "{\n        false\n    }",
                                                    },
                                                },
                                            },
                                        },
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 216,
                                                            end: 218,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 219,
                                                            end: 221,
                                                            as_str(): "gt",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 222,
                                                                    end: 226,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 226,
                                                                            end: 227,
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
                                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                            ),
                                                                                            start: 228,
                                                                                            end: 233,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                        ),
                                                                                        start: 233,
                                                                                        end: 234,
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
                                                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                                    ),
                                                                                                    start: 235,
                                                                                                    end: 239,
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
                                                            src (ptr): 0x00007fe0bc9cf310,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                            ),
                                                            start: 221,
                                                            end: 240,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 241,
                                                                    end: 243,
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
                                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                                ),
                                                                                start: 244,
                                                                                end: 248,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 264,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 249,
                                                        end: 270,
                                                        as_str(): "{\n        false\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 272,
                                        as_str(): "{\n    fn lt(self, other: Self) -> bool {\n        false\n    }\n    fn gt(self, other: Self) -> bool {\n        false\n    }\n}",
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
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 274,
                                            end: 276,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 277,
                                            end: 281,
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
                                            src (ptr): 0x00007fe0bc9cf310,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                            ),
                                            start: 281,
                                            end: 283,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0bc9cf310,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                    ),
                                                    start: 284,
                                                    end: 286,
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
                                                                src (ptr): 0x00007fe0bc9cf310,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                ),
                                                                start: 287,
                                                                end: 291,
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
                                            Equal {
                                                lhs: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 298,
                                                                    end: 299,
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
                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 301,
                                                                            end: 302,
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
                                                double_eq_token: DoubleEqToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0bc9cf310,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                        ),
                                                        start: 303,
                                                        end: 305,
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
                                                                    src (ptr): 0x00007fe0bc9cf310,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                    ),
                                                                    start: 306,
                                                                    end: 307,
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
                                                                        src (ptr): 0x00007fe0bc9cf310,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                        ),
                                                                        start: 307,
                                                                        end: 309,
                                                                        as_str(): "::",
                                                                    },
                                                                },
                                                                PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0bc9cf310,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                                                            ),
                                                                            start: 309,
                                                                            end: 310,
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
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0bc9cf310,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRsStL2j/local_impl_for_ord/src/main.sw",
                                        ),
                                        start: 292,
                                        end: 312,
                                        as_str(): "{\n    X::Y == X::Y\n}",
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
