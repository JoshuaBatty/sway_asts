Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
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
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
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
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 24,
                                                end: 26,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 32,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 32,
                                        end: 33,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 40,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 46,
                                        as_str(): "MyAdd",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 53,
                                                            end: 55,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 56,
                                                            end: 62,
                                                            as_str(): "my_add",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 67,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 67,
                                                                            end: 68,
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
                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                            ),
                                                                                            start: 69,
                                                                                            end: 74,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 74,
                                                                                        end: 75,
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 76,
                                                                                                    end: 80,
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 62,
                                                            end: 81,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 82,
                                                                    end: 84,
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
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 85,
                                                                                end: 89,
                                                                                as_str(): "Self",
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
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 89,
                                                    end: 90,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 47,
                                        end: 92,
                                        as_str(): "{\n    fn my_add(self, other: Self) -> Self;\n}",
                                    },
                                },
                                trait_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 94,
                                        end: 99,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 100,
                                        end: 105,
                                        as_str(): "MyMul",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 114,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 115,
                                                            end: 121,
                                                            as_str(): "my_mul",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 122,
                                                                    end: 126,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 126,
                                                                            end: 127,
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
                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                            ),
                                                                                            start: 128,
                                                                                            end: 133,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 133,
                                                                                        end: 134,
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 135,
                                                                                                    end: 139,
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 121,
                                                            end: 140,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 141,
                                                                    end: 143,
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
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 144,
                                                                                end: 148,
                                                                                as_str(): "Self",
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
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 148,
                                                    end: 149,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 106,
                                        end: 151,
                                        as_str(): "{\n    fn my_mul(self, other: Self) -> Self;\n}",
                                    },
                                },
                                trait_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 153,
                                        end: 158,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 159,
                                        end: 165,
                                        as_str(): "MyMath",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 165,
                                                end: 166,
                                                as_str(): ":",
                                            },
                                        },
                                        Traits {
                                            prefix: PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 167,
                                                            end: 172,
                                                            as_str(): "MyAdd",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                            suffixes: [
                                                (
                                                    AddToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 173,
                                                            end: 174,
                                                            as_str(): "+",
                                                        },
                                                    },
                                                    PathType {
                                                        root_opt: None,
                                                        prefix: PathTypeSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 180,
                                                                    as_str(): "MyMul",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                    },
                                                ),
                                            ],
                                        },
                                    ),
                                ),
                                trait_items: Braces {
                                    inner: [],
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 181,
                                        end: 185,
                                        as_str(): "{\n\n}",
                                    },
                                },
                                trait_defs_opt: Some(
                                    Braces {
                                        inner: [
                                            Annotated {
                                                attribute_list: [],
                                                value: ItemFn {
                                                    fn_signature: FnSignature {
                                                        visibility: None,
                                                        fn_token: FnToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 192,
                                                                end: 194,
                                                                as_str(): "fn",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 195,
                                                                end: 204,
                                                                as_str(): "my_double",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics: None,
                                                        arguments: Parens {
                                                            inner: NonStatic {
                                                                self_token: SelfToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 209,
                                                                        as_str(): "self",
                                                                    },
                                                                },
                                                                ref_self: None,
                                                                mutable_self: None,
                                                                args_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 204,
                                                                end: 210,
                                                                as_str(): "(self)",
                                                            },
                                                        },
                                                        return_type_opt: Some(
                                                            (
                                                                RightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 211,
                                                                        end: 213,
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
                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                    ),
                                                                                    start: 214,
                                                                                    end: 218,
                                                                                    as_str(): "Self",
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
                                                                MethodCall {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 229,
                                                                                        end: 233,
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
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 233,
                                                                            end: 234,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 234,
                                                                                end: 240,
                                                                                as_str(): "my_add",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    contract_args_opt: None,
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 241,
                                                                                                    end: 245,
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
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 240,
                                                                            end: 246,
                                                                            as_str(): "(self)",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 219,
                                                            end: 252,
                                                            as_str(): "{\n        self.my_add(self)\n    }",
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
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 258,
                                                                end: 260,
                                                                as_str(): "fn",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 261,
                                                                end: 267,
                                                                as_str(): "my_exp",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics: None,
                                                        arguments: Parens {
                                                            inner: NonStatic {
                                                                self_token: SelfToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 268,
                                                                        end: 272,
                                                                        as_str(): "self",
                                                                    },
                                                                },
                                                                ref_self: None,
                                                                mutable_self: None,
                                                                args_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 267,
                                                                end: 273,
                                                                as_str(): "(self)",
                                                            },
                                                        },
                                                        return_type_opt: Some(
                                                            (
                                                                RightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 274,
                                                                        end: 276,
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
                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                    ),
                                                                                    start: 277,
                                                                                    end: 281,
                                                                                    as_str(): "Self",
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
                                                                MethodCall {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 292,
                                                                                        end: 296,
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
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 296,
                                                                            end: 297,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 297,
                                                                                end: 303,
                                                                                as_str(): "my_mul",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    contract_args_opt: None,
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 304,
                                                                                                    end: 308,
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
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 309,
                                                                            as_str(): "(self)",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 315,
                                                            as_str(): "{\n        self.my_mul(self)\n    }",
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 317,
                                            as_str(): "{\n    fn my_double(self) -> Self {\n        self.my_add(self)\n    }\n\n    fn my_exp(self) -> Self {\n        self.my_mul(self)\n    }\n}",
                                        },
                                    },
                                ),
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 319,
                                        end: 325,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 326,
                                        end: 330,
                                        as_str(): "Data",
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
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 337,
                                                                end: 342,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 342,
                                                                end: 343,
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
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 344,
                                                                            end: 347,
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
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 347,
                                                        end: 348,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 331,
                                        end: 350,
                                        as_str(): "{\n    value: u64,\n}",
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 352,
                                        end: 356,
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
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 357,
                                                        end: 362,
                                                        as_str(): "MyAdd",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 363,
                                                end: 366,
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
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 367,
                                                    end: 371,
                                                    as_str(): "Data",
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 378,
                                                            end: 380,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 381,
                                                            end: 387,
                                                            as_str(): "my_add",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 388,
                                                                    end: 392,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 392,
                                                                            end: 393,
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
                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                            ),
                                                                                            start: 394,
                                                                                            end: 399,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 399,
                                                                                        end: 400,
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 401,
                                                                                                    end: 405,
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 387,
                                                            end: 406,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 407,
                                                                    end: 409,
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
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 410,
                                                                                end: 414,
                                                                                as_str(): "Self",
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
                                                            Struct {
                                                                path: PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 425,
                                                                                end: 429,
                                                                                as_str(): "Data",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                                fields: Braces {
                                                                    inner: Punctuated {
                                                                        value_separator_pairs: [],
                                                                        final_value_opt: Some(
                                                                            ExprStructField {
                                                                                field_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 444,
                                                                                        end: 449,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                expr_opt: Some(
                                                                                    (
                                                                                        ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                ),
                                                                                                start: 449,
                                                                                                end: 450,
                                                                                                as_str(): ":",
                                                                                            },
                                                                                        },
                                                                                        Add {
                                                                                            lhs: FieldProjection {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 451,
                                                                                                                    end: 455,
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
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 455,
                                                                                                        end: 456,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 456,
                                                                                                        end: 461,
                                                                                                        as_str(): "value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            add_token: AddToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 462,
                                                                                                    end: 463,
                                                                                                    as_str(): "+",
                                                                                                },
                                                                                            },
                                                                                            rhs: FieldProjection {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 464,
                                                                                                                    end: 469,
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
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 469,
                                                                                                        end: 470,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 470,
                                                                                                        end: 475,
                                                                                                        as_str(): "value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 430,
                                                                        end: 485,
                                                                        as_str(): "{\n            value: self.value + other.value\n        }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 415,
                                                        end: 491,
                                                        as_str(): "{\n        Data {\n            value: self.value + other.value\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 372,
                                        end: 493,
                                        as_str(): "{\n    fn my_add(self, other: Self) -> Self {\n        Data {\n            value: self.value + other.value\n        }\n    }\n}",
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 495,
                                        end: 499,
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
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 500,
                                                        end: 505,
                                                        as_str(): "MyMul",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 506,
                                                end: 509,
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
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 510,
                                                    end: 514,
                                                    as_str(): "Data",
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 521,
                                                            end: 523,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 524,
                                                            end: 530,
                                                            as_str(): "my_mul",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 531,
                                                                    end: 535,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 535,
                                                                            end: 536,
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
                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                            ),
                                                                                            start: 537,
                                                                                            end: 542,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 542,
                                                                                        end: 543,
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
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 544,
                                                                                                    end: 548,
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 530,
                                                            end: 549,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 550,
                                                                    end: 552,
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
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 553,
                                                                                end: 557,
                                                                                as_str(): "Self",
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
                                                            Struct {
                                                                path: PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 568,
                                                                                end: 572,
                                                                                as_str(): "Data",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                                fields: Braces {
                                                                    inner: Punctuated {
                                                                        value_separator_pairs: [],
                                                                        final_value_opt: Some(
                                                                            ExprStructField {
                                                                                field_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 587,
                                                                                        end: 592,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                expr_opt: Some(
                                                                                    (
                                                                                        ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                ),
                                                                                                start: 592,
                                                                                                end: 593,
                                                                                                as_str(): ":",
                                                                                            },
                                                                                        },
                                                                                        Mul {
                                                                                            lhs: FieldProjection {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 594,
                                                                                                                    end: 598,
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
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 598,
                                                                                                        end: 599,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 599,
                                                                                                        end: 604,
                                                                                                        as_str(): "value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            star_token: StarToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                    ),
                                                                                                    start: 605,
                                                                                                    end: 606,
                                                                                                    as_str(): "*",
                                                                                                },
                                                                                            },
                                                                                            rhs: FieldProjection {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 607,
                                                                                                                    end: 612,
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
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 612,
                                                                                                        end: 613,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 613,
                                                                                                        end: 618,
                                                                                                        as_str(): "value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 573,
                                                                        end: 628,
                                                                        as_str(): "{\n            value: self.value * other.value\n        }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 558,
                                                        end: 634,
                                                        as_str(): "{\n        Data {\n            value: self.value * other.value\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 515,
                                        end: 636,
                                        as_str(): "{\n    fn my_mul(self, other: Self) -> Self {\n        Data {\n            value: self.value * other.value\n        }\n    }\n}",
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 638,
                                        end: 642,
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
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 643,
                                                        end: 649,
                                                        as_str(): "MyMath",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 650,
                                                end: 653,
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
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 654,
                                                    end: 658,
                                                    as_str(): "Data",
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
                                    inner: [],
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 659,
                                        end: 661,
                                        as_str(): "{}",
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
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 663,
                                            end: 665,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 666,
                                            end: 670,
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
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 670,
                                            end: 672,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 673,
                                                    end: 675,
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
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 676,
                                                                end: 680,
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 687,
                                                            end: 690,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 691,
                                                                end: 692,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 693,
                                                            end: 694,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 695,
                                                                        end: 699,
                                                                        as_str(): "Data",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                        fields: Braces {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    ExprStructField {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 710,
                                                                                end: 715,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                        ),
                                                                                        start: 715,
                                                                                        end: 716,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                ),
                                                                                                start: 717,
                                                                                                end: 718,
                                                                                                as_str(): "3",
                                                                                            },
                                                                                            parsed: 3,
                                                                                            ty_opt: Some(
                                                                                                (
                                                                                                    U64,
                                                                                                    Span {
                                                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                                        ),
                                                                                                        start: 718,
                                                                                                        end: 721,
                                                                                                        as_str(): "u64",
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 700,
                                                                end: 727,
                                                                as_str(): "{\n        value: 3u64\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 727,
                                                            end: 728,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 733,
                                                            end: 736,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 737,
                                                                end: 738,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 739,
                                                            end: 740,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: MethodCall {
                                                        target: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 741,
                                                                            end: 742,
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
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 742,
                                                                end: 743,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        path_seg: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 743,
                                                                    end: 749,
                                                                    as_str(): "my_exp",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        contract_args_opt: None,
                                                        args: Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 749,
                                                                end: 751,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 751,
                                                            end: 752,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 757,
                                                            end: 760,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 761,
                                                                end: 762,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 763,
                                                            end: 764,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: MethodCall {
                                                        target: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 765,
                                                                            end: 766,
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
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 766,
                                                                end: 767,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        path_seg: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 767,
                                                                    end: 776,
                                                                    as_str(): "my_double",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        contract_args_opt: None,
                                                        args: Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 776,
                                                                end: 778,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 778,
                                                            end: 779,
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
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 784,
                                                                        end: 790,
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
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                            ),
                                                                                            start: 791,
                                                                                            end: 792,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 792,
                                                                                end: 793,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe02ab473d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                ),
                                                                                start: 793,
                                                                                end: 798,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 799,
                                                                            end: 801,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                                    ),
                                                                                    start: 802,
                                                                                    end: 804,
                                                                                    as_str(): "18",
                                                                                },
                                                                                parsed: 18,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 790,
                                                            end: 805,
                                                            as_str(): "(c.value == 18)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 805,
                                                            end: 806,
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 812,
                                                            end: 816,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 681,
                                        end: 818,
                                        as_str(): "{\n    let a = Data {\n        value: 3u64\n    };\n    let b = a.my_exp();\n    let c = b.my_double();\n    assert(c.value == 18);\n\n    true\n}",
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
