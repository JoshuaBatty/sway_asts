Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fb117fe0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 8,
                                        end: 11,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Glob {
                                        star_token: StarToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 19,
                                                as_str(): "*",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 19,
                                        end: 20,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 21,
                                        end: 24,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 25,
                                            end: 29,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 29,
                                            end: 31,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 31,
                                                end: 34,
                                                as_str(): "ops",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 34,
                                                end: 36,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 36,
                                                    end: 39,
                                                    as_str(): "Ord",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 39,
                                        end: 40,
                                        as_str(): ";",
                                    },
                                },
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 42,
                                        end: 48,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 49,
                                        end: 52,
                                        as_str(): "Rgb",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 59,
                                                                end: 62,
                                                                as_str(): "red",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 62,
                                                                end: 63,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 64,
                                                                            end: 67,
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 67,
                                                        end: 68,
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 78,
                                                                as_str(): "green",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 78,
                                                                end: 79,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 80,
                                                                            end: 83,
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 83,
                                                        end: 84,
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 89,
                                                                end: 93,
                                                                as_str(): "blue",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 93,
                                                                end: 94,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 95,
                                                                            end: 98,
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 98,
                                                        end: 99,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 101,
                                        as_str(): "{\n    red: u64,\n    green: u64,\n    blue: u64,\n}",
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 103,
                                        end: 108,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 109,
                                        end: 114,
                                        as_str(): "Color",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 121,
                                                            end: 123,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 124,
                                                            end: 127,
                                                            as_str(): "rgb",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 132,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 127,
                                                            end: 133,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 136,
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
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 137,
                                                                                end: 140,
                                                                                as_str(): "Rgb",
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 140,
                                                    end: 141,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 115,
                                        end: 143,
                                        as_str(): "{\n    fn rgb(self) -> Rgb;\n}",
                                    },
                                },
                                trait_defs_opt: None,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 145,
                                        end: 149,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 162,
                                        as_str(): "PrimaryColor",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 169,
                                                                end: 172,
                                                                as_str(): "Red",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 172,
                                                                end: 173,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 174,
                                                                    end: 176,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 176,
                                                        end: 177,
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 182,
                                                                end: 187,
                                                                as_str(): "Green",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 187,
                                                                end: 188,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 189,
                                                                    end: 191,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 191,
                                                        end: 192,
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 201,
                                                                as_str(): "Blue",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 201,
                                                                end: 202,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 203,
                                                                    end: 205,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 205,
                                                        end: 206,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 163,
                                        end: 208,
                                        as_str(): "{\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 210,
                                        end: 214,
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 215,
                                                        end: 219,
                                                        as_str(): "core",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [
                                                (
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 219,
                                                            end: 221,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 221,
                                                                end: 224,
                                                                as_str(): "ops",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                ),
                                                (
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 224,
                                                            end: 226,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 226,
                                                                end: 228,
                                                                as_str(): "Eq",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                ),
                                            ],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 229,
                                                end: 232,
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 233,
                                                    end: 245,
                                                    as_str(): "PrimaryColor",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 252,
                                                            end: 254,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 255,
                                                            end: 257,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 258,
                                                                    end: 262,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 262,
                                                                            end: 263,
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
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 264,
                                                                                            end: 269,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 269,
                                                                                        end: 270,
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
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 271,
                                                                                                    end: 275,
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 257,
                                                            end: 276,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 277,
                                                                    end: 279,
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
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 280,
                                                                                end: 284,
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
                                                            Asm(
                                                                AsmBlock {
                                                                    asm_token: AsmToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 295,
                                                                            end: 298,
                                                                            as_str(): "asm",
                                                                        },
                                                                    },
                                                                    registers: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 299,
                                                                                                end: 301,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 301,
                                                                                                        end: 302,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 303,
                                                                                                                    end: 307,
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
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 307,
                                                                                            end: 308,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 309,
                                                                                                end: 311,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 311,
                                                                                                        end: 312,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 313,
                                                                                                                    end: 318,
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
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 318,
                                                                                            end: 319,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                AsmRegisterDeclaration {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 320,
                                                                                            end: 322,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 298,
                                                                            end: 323,
                                                                            as_str(): "(r1: self, r2: other, r3)",
                                                                        },
                                                                    },
                                                                    contents: Braces {
                                                                        inner: AsmBlockContents {
                                                                            instructions: [
                                                                                (
                                                                                    Eq {
                                                                                        token: EqOpcode {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 338,
                                                                                                end: 340,
                                                                                                as_str(): "eq",
                                                                                            },
                                                                                        },
                                                                                        ret: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 341,
                                                                                                end: 343,
                                                                                                as_str(): "r3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        lhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 344,
                                                                                                end: 346,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        rhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 347,
                                                                                                end: 349,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 349,
                                                                                            end: 350,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_expr_opt: Some(
                                                                                AsmFinalExpr {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 363,
                                                                                            end: 365,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 365,
                                                                                                    end: 366,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 367,
                                                                                                                end: 371,
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
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 324,
                                                                            end: 381,
                                                                            as_str(): "{\n            eq r3 r1 r2;\n            r3: bool\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 285,
                                                        end: 387,
                                                        as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 246,
                                        end: 389,
                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 391,
                                        end: 395,
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 396,
                                                        end: 400,
                                                        as_str(): "core",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [
                                                (
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 400,
                                                            end: 402,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 402,
                                                                end: 405,
                                                                as_str(): "ops",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                ),
                                                (
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 405,
                                                            end: 407,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 407,
                                                                end: 410,
                                                                as_str(): "Ord",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                ),
                                            ],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 411,
                                                end: 414,
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 415,
                                                    end: 427,
                                                    as_str(): "PrimaryColor",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 434,
                                                            end: 436,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 437,
                                                            end: 439,
                                                            as_str(): "lt",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 440,
                                                                    end: 444,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 444,
                                                                            end: 445,
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
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 446,
                                                                                            end: 451,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 451,
                                                                                        end: 452,
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
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 453,
                                                                                                    end: 457,
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 439,
                                                            end: 458,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 459,
                                                                    end: 461,
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
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 462,
                                                                                end: 466,
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
                                                            Asm(
                                                                AsmBlock {
                                                                    asm_token: AsmToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 477,
                                                                            end: 480,
                                                                            as_str(): "asm",
                                                                        },
                                                                    },
                                                                    registers: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 481,
                                                                                                end: 483,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 483,
                                                                                                        end: 484,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 485,
                                                                                                                    end: 489,
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
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 489,
                                                                                            end: 490,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 491,
                                                                                                end: 493,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 493,
                                                                                                        end: 494,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 495,
                                                                                                                    end: 500,
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
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 500,
                                                                                            end: 501,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                AsmRegisterDeclaration {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 502,
                                                                                            end: 504,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 480,
                                                                            end: 505,
                                                                            as_str(): "(r1: self, r2: other, r3)",
                                                                        },
                                                                    },
                                                                    contents: Braces {
                                                                        inner: AsmBlockContents {
                                                                            instructions: [
                                                                                (
                                                                                    Lt {
                                                                                        token: LtOpcode {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 520,
                                                                                                end: 522,
                                                                                                as_str(): "lt",
                                                                                            },
                                                                                        },
                                                                                        ret: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 523,
                                                                                                end: 525,
                                                                                                as_str(): "r3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        lhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 526,
                                                                                                end: 528,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        rhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 529,
                                                                                                end: 531,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 531,
                                                                                            end: 532,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_expr_opt: Some(
                                                                                AsmFinalExpr {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 545,
                                                                                            end: 547,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 547,
                                                                                                    end: 548,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 549,
                                                                                                                end: 553,
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
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 506,
                                                                            end: 563,
                                                                            as_str(): "{\n            lt r3 r1 r2;\n            r3: bool\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 467,
                                                        end: 569,
                                                        as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 574,
                                                            end: 576,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 577,
                                                            end: 579,
                                                            as_str(): "gt",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 580,
                                                                    end: 584,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 584,
                                                                            end: 585,
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
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 586,
                                                                                            end: 591,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 591,
                                                                                        end: 592,
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
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 593,
                                                                                                    end: 597,
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 579,
                                                            end: 598,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 599,
                                                                    end: 601,
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
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 602,
                                                                                end: 606,
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
                                                            Asm(
                                                                AsmBlock {
                                                                    asm_token: AsmToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 617,
                                                                            end: 620,
                                                                            as_str(): "asm",
                                                                        },
                                                                    },
                                                                    registers: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 621,
                                                                                                end: 623,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 623,
                                                                                                        end: 624,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 625,
                                                                                                                    end: 629,
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
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 629,
                                                                                            end: 630,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 631,
                                                                                                end: 633,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 633,
                                                                                                        end: 634,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 635,
                                                                                                                    end: 640,
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
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 640,
                                                                                            end: 641,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                AsmRegisterDeclaration {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 642,
                                                                                            end: 644,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 620,
                                                                            end: 645,
                                                                            as_str(): "(r1: self, r2: other, r3)",
                                                                        },
                                                                    },
                                                                    contents: Braces {
                                                                        inner: AsmBlockContents {
                                                                            instructions: [
                                                                                (
                                                                                    Gt {
                                                                                        token: GtOpcode {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 660,
                                                                                                end: 662,
                                                                                                as_str(): "gt",
                                                                                            },
                                                                                        },
                                                                                        ret: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 663,
                                                                                                end: 665,
                                                                                                as_str(): "r3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        lhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 666,
                                                                                                end: 668,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        rhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 669,
                                                                                                end: 671,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 671,
                                                                                            end: 672,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_expr_opt: Some(
                                                                                AsmFinalExpr {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 685,
                                                                                            end: 687,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 687,
                                                                                                    end: 688,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 689,
                                                                                                                end: 693,
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
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 646,
                                                                            end: 703,
                                                                            as_str(): "{\n            gt r3 r1 r2;\n            r3: bool\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 607,
                                                        end: 709,
                                                        as_str(): "{\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 428,
                                        end: 711,
                                        as_str(): "{\n    fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }\n    fn gt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 713,
                                        end: 717,
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 718,
                                                        end: 723,
                                                        as_str(): "Color",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 724,
                                                end: 727,
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 728,
                                                    end: 740,
                                                    as_str(): "PrimaryColor",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 827,
                                                            end: 829,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 830,
                                                            end: 833,
                                                            as_str(): "rgb",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 834,
                                                                    end: 838,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 833,
                                                            end: 839,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 840,
                                                                    end: 842,
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
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 843,
                                                                                end: 846,
                                                                                as_str(): "Rgb",
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 857,
                                                                            end: 859,
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
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 860,
                                                                                                end: 864,
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
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 865,
                                                                                    end: 867,
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
                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 868,
                                                                                                end: 880,
                                                                                                as_str(): "PrimaryColor",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 880,
                                                                                                    end: 882,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 882,
                                                                                                        end: 885,
                                                                                                        as_str(): "Red",
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
                                                                    then_block: Braces {
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
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 900,
                                                                                                    end: 903,
                                                                                                    as_str(): "Rgb",
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
                                                                                            value_separator_pairs: [
                                                                                                (
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 922,
                                                                                                                end: 925,
                                                                                                                as_str(): "red",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 925,
                                                                                                                        end: 926,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 927,
                                                                                                                                end: 930,
                                                                                                                                as_str(): "255",
                                                                                                                            },
                                                                                                                            parsed: 255,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 930,
                                                                                                            end: 931,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 948,
                                                                                                                end: 952,
                                                                                                                as_str(): "blue",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 952,
                                                                                                                        end: 953,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 954,
                                                                                                                                end: 955,
                                                                                                                                as_str(): "0",
                                                                                                                            },
                                                                                                                            parsed: 0,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 955,
                                                                                                            end: 956,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 973,
                                                                                                                end: 978,
                                                                                                                as_str(): "green",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 978,
                                                                                                                        end: 979,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 980,
                                                                                                                                end: 981,
                                                                                                                                as_str(): "0",
                                                                                                                            },
                                                                                                                            parsed: 0,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 981,
                                                                                                            end: 982,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 904,
                                                                                            end: 996,
                                                                                            as_str(): "{\n                red: 255,\n                blue: 0,\n                green: 0,\n            }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 886,
                                                                            end: 1006,
                                                                            as_str(): "{\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        }",
                                                                        },
                                                                    },
                                                                    else_opt: Some(
                                                                        (
                                                                            ElseToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 1007,
                                                                                    end: 1011,
                                                                                    as_str(): "else",
                                                                                },
                                                                            },
                                                                            Continue(
                                                                                IfExpr {
                                                                                    if_token: IfToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 1012,
                                                                                            end: 1014,
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
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1015,
                                                                                                                end: 1019,
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
                                                                                            double_eq_token: DoubleEqToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1020,
                                                                                                    end: 1022,
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
                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1023,
                                                                                                                end: 1035,
                                                                                                                as_str(): "PrimaryColor",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [
                                                                                                        (
                                                                                                            DoubleColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1035,
                                                                                                                    end: 1037,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1037,
                                                                                                                        end: 1042,
                                                                                                                        as_str(): "Green",
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
                                                                                    then_block: Braces {
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
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1057,
                                                                                                                    end: 1060,
                                                                                                                    as_str(): "Rgb",
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
                                                                                                            value_separator_pairs: [
                                                                                                                (
                                                                                                                    ExprStructField {
                                                                                                                        field_name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1079,
                                                                                                                                end: 1082,
                                                                                                                                as_str(): "red",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        expr_opt: Some(
                                                                                                                            (
                                                                                                                                ColonToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1082,
                                                                                                                                        end: 1083,
                                                                                                                                        as_str(): ":",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                Literal(
                                                                                                                                    Int(
                                                                                                                                        LitInt {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1084,
                                                                                                                                                end: 1085,
                                                                                                                                                as_str(): "0",
                                                                                                                                            },
                                                                                                                                            parsed: 0,
                                                                                                                                            ty_opt: None,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    CommaToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1085,
                                                                                                                            end: 1086,
                                                                                                                            as_str(): ",",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    ExprStructField {
                                                                                                                        field_name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1103,
                                                                                                                                end: 1107,
                                                                                                                                as_str(): "blue",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        expr_opt: Some(
                                                                                                                            (
                                                                                                                                ColonToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1107,
                                                                                                                                        end: 1108,
                                                                                                                                        as_str(): ":",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                Literal(
                                                                                                                                    Int(
                                                                                                                                        LitInt {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1109,
                                                                                                                                                end: 1110,
                                                                                                                                                as_str(): "0",
                                                                                                                                            },
                                                                                                                                            parsed: 0,
                                                                                                                                            ty_opt: None,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    CommaToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1110,
                                                                                                                            end: 1111,
                                                                                                                            as_str(): ",",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    ExprStructField {
                                                                                                                        field_name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1128,
                                                                                                                                end: 1133,
                                                                                                                                as_str(): "green",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        expr_opt: Some(
                                                                                                                            (
                                                                                                                                ColonToken {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1133,
                                                                                                                                        end: 1134,
                                                                                                                                        as_str(): ":",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                Literal(
                                                                                                                                    Int(
                                                                                                                                        LitInt {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1135,
                                                                                                                                                end: 1138,
                                                                                                                                                as_str(): "255",
                                                                                                                                            },
                                                                                                                                            parsed: 255,
                                                                                                                                            ty_opt: None,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    CommaToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1138,
                                                                                                                            end: 1139,
                                                                                                                            as_str(): ",",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            final_value_opt: None,
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1061,
                                                                                                            end: 1153,
                                                                                                            as_str(): "{\n                red: 0,\n                blue: 0,\n                green: 255,\n            }",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 1043,
                                                                                            end: 1163,
                                                                                            as_str(): "{\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        }",
                                                                                        },
                                                                                    },
                                                                                    else_opt: Some(
                                                                                        (
                                                                                            ElseToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1164,
                                                                                                    end: 1168,
                                                                                                    as_str(): "else",
                                                                                                },
                                                                                            },
                                                                                            Continue(
                                                                                                IfExpr {
                                                                                                    if_token: IfToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1169,
                                                                                                            end: 1171,
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
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1172,
                                                                                                                                end: 1176,
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
                                                                                                            double_eq_token: DoubleEqToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1177,
                                                                                                                    end: 1179,
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
                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1180,
                                                                                                                                end: 1192,
                                                                                                                                as_str(): "PrimaryColor",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [
                                                                                                                        (
                                                                                                                            DoubleColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1192,
                                                                                                                                    end: 1194,
                                                                                                                                    as_str(): "::",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            PathExprSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1194,
                                                                                                                                        end: 1198,
                                                                                                                                        as_str(): "Blue",
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
                                                                                                    then_block: Braces {
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
                                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1213,
                                                                                                                                    end: 1216,
                                                                                                                                    as_str(): "Rgb",
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
                                                                                                                            value_separator_pairs: [
                                                                                                                                (
                                                                                                                                    ExprStructField {
                                                                                                                                        field_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1235,
                                                                                                                                                end: 1238,
                                                                                                                                                as_str(): "red",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1238,
                                                                                                                                                        end: 1239,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 1240,
                                                                                                                                                                end: 1241,
                                                                                                                                                                as_str(): "0",
                                                                                                                                                            },
                                                                                                                                                            parsed: 0,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1241,
                                                                                                                                            end: 1242,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                (
                                                                                                                                    ExprStructField {
                                                                                                                                        field_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1259,
                                                                                                                                                end: 1263,
                                                                                                                                                as_str(): "blue",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1263,
                                                                                                                                                        end: 1264,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 1265,
                                                                                                                                                                end: 1268,
                                                                                                                                                                as_str(): "255",
                                                                                                                                                            },
                                                                                                                                                            parsed: 255,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1268,
                                                                                                                                            end: 1269,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                (
                                                                                                                                    ExprStructField {
                                                                                                                                        field_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1286,
                                                                                                                                                end: 1291,
                                                                                                                                                as_str(): "green",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1291,
                                                                                                                                                        end: 1292,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 1293,
                                                                                                                                                                end: 1294,
                                                                                                                                                                as_str(): "0",
                                                                                                                                                            },
                                                                                                                                                            parsed: 0,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1294,
                                                                                                                                            end: 1295,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ],
                                                                                                                            final_value_opt: None,
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1217,
                                                                                                                            end: 1309,
                                                                                                                            as_str(): "{\n                red: 0,\n                blue: 255,\n                green: 0,\n            }",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1199,
                                                                                                            end: 1319,
                                                                                                            as_str(): "{\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }",
                                                                                                        },
                                                                                                    },
                                                                                                    else_opt: Some(
                                                                                                        (
                                                                                                            ElseToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1397,
                                                                                                                    end: 1401,
                                                                                                                    as_str(): "else",
                                                                                                                },
                                                                                                            },
                                                                                                            Break(
                                                                                                                Braces {
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
                                                                                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 1416,
                                                                                                                                                end: 1419,
                                                                                                                                                as_str(): "Rgb",
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
                                                                                                                                        value_separator_pairs: [
                                                                                                                                            (
                                                                                                                                                ExprStructField {
                                                                                                                                                    field_name: BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1438,
                                                                                                                                                            end: 1441,
                                                                                                                                                            as_str(): "red",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    expr_opt: Some(
                                                                                                                                                        (
                                                                                                                                                            ColonToken {
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 1441,
                                                                                                                                                                    end: 1442,
                                                                                                                                                                    as_str(): ":",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            Literal(
                                                                                                                                                                Int(
                                                                                                                                                                    LitInt {
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 1443,
                                                                                                                                                                            end: 1444,
                                                                                                                                                                            as_str(): "0",
                                                                                                                                                                        },
                                                                                                                                                                        parsed: 0,
                                                                                                                                                                        ty_opt: None,
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                    ),
                                                                                                                                                },
                                                                                                                                                CommaToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1444,
                                                                                                                                                        end: 1445,
                                                                                                                                                        as_str(): ",",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            (
                                                                                                                                                ExprStructField {
                                                                                                                                                    field_name: BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1462,
                                                                                                                                                            end: 1467,
                                                                                                                                                            as_str(): "green",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    expr_opt: Some(
                                                                                                                                                        (
                                                                                                                                                            ColonToken {
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 1467,
                                                                                                                                                                    end: 1468,
                                                                                                                                                                    as_str(): ":",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            Literal(
                                                                                                                                                                Int(
                                                                                                                                                                    LitInt {
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 1469,
                                                                                                                                                                            end: 1470,
                                                                                                                                                                            as_str(): "0",
                                                                                                                                                                        },
                                                                                                                                                                        parsed: 0,
                                                                                                                                                                        ty_opt: None,
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                    ),
                                                                                                                                                },
                                                                                                                                                CommaToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1470,
                                                                                                                                                        end: 1471,
                                                                                                                                                        as_str(): ",",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                            (
                                                                                                                                                ExprStructField {
                                                                                                                                                    field_name: BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1488,
                                                                                                                                                            end: 1492,
                                                                                                                                                            as_str(): "blue",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    expr_opt: Some(
                                                                                                                                                        (
                                                                                                                                                            ColonToken {
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 1492,
                                                                                                                                                                    end: 1493,
                                                                                                                                                                    as_str(): ":",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            Literal(
                                                                                                                                                                Int(
                                                                                                                                                                    LitInt {
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 1494,
                                                                                                                                                                            end: 1495,
                                                                                                                                                                            as_str(): "0",
                                                                                                                                                                        },
                                                                                                                                                                        parsed: 0,
                                                                                                                                                                        ty_opt: None,
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                    ),
                                                                                                                                                },
                                                                                                                                                CommaToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1495,
                                                                                                                                                        end: 1496,
                                                                                                                                                        as_str(): ",",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        ],
                                                                                                                                        final_value_opt: None,
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1420,
                                                                                                                                        end: 1510,
                                                                                                                                        as_str(): "{\n                red: 0,\n                green: 0,\n                blue: 0,\n            }",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1402,
                                                                                                                        end: 1520,
                                                                                                                        as_str(): "{\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }",
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
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 847,
                                                        end: 1526,
                                                        as_str(): "{\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 741,
                                        end: 1528,
                                        as_str(): "{\n    // TODO: when we support match statements, change this to a match statement\n    fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }\n}",
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
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 1530,
                                            end: 1532,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 1533,
                                            end: 1537,
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
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 1537,
                                            end: 1539,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1540,
                                                    end: 1542,
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1543,
                                                                end: 1546,
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1553,
                                                            end: 1556,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1557,
                                                                end: 1568,
                                                                as_str(): "first_color",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1568,
                                                                    end: 1569,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1570,
                                                                                end: 1582,
                                                                                as_str(): "PrimaryColor",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1583,
                                                            end: 1584,
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
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1585,
                                                                        end: 1597,
                                                                        as_str(): "PrimaryColor",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1597,
                                                                            end: 1599,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1599,
                                                                                end: 1604,
                                                                                as_str(): "Green",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1604,
                                                            end: 1605,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1610,
                                                            end: 1613,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1614,
                                                                end: 1618,
                                                                as_str(): "test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1619,
                                                            end: 1620,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1621,
                                                                            end: 1632,
                                                                            as_str(): "first_color",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1633,
                                                                end: 1635,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1636,
                                                                            end: 1648,
                                                                            as_str(): "PrimaryColor",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1648,
                                                                                end: 1650,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 1650,
                                                                                    end: 1655,
                                                                                    as_str(): "Green",
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1655,
                                                            end: 1656,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1741,
                                                            end: 1744,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1745,
                                                                end: 1748,
                                                                as_str(): "rgb",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1748,
                                                                    end: 1749,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1750,
                                                                                end: 1753,
                                                                                as_str(): "Rgb",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1754,
                                                            end: 1755,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1756,
                                                                            end: 1767,
                                                                            as_str(): "first_color",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1767,
                                                                end: 1768,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        path_seg: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1768,
                                                                    end: 1771,
                                                                    as_str(): "rgb",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1771,
                                                                end: 1773,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1773,
                                                            end: 1774,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1850,
                                                            end: 1853,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1854,
                                                                end: 1866,
                                                                as_str(): "second_color",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1867,
                                                            end: 1868,
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
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1869,
                                                                        end: 1881,
                                                                        as_str(): "PrimaryColor",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1881,
                                                                            end: 1883,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1883,
                                                                                end: 1887,
                                                                                as_str(): "Blue",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1887,
                                                            end: 1888,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1893,
                                                            end: 1896,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1897,
                                                                end: 1907,
                                                                as_str(): "second_rgb",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1908,
                                                            end: 1909,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1910,
                                                                            end: 1922,
                                                                            as_str(): "second_color",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1922,
                                                                end: 1923,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        path_seg: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 1923,
                                                                    end: 1926,
                                                                    as_str(): "rgb",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1926,
                                                                end: 1928,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1928,
                                                            end: 1929,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1934,
                                                            end: 1937,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1938,
                                                                end: 1950,
                                                                as_str(): "second_color",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1951,
                                                            end: 1952,
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
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 1953,
                                                                        end: 1965,
                                                                        as_str(): "PrimaryColor",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1965,
                                                                            end: 1967,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 1967,
                                                                                end: 1971,
                                                                                as_str(): "Blue",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1971,
                                                            end: 1972,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1977,
                                                            end: 1980,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 1981,
                                                                end: 1991,
                                                                as_str(): "second_rgb",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1992,
                                                            end: 1993,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 1994,
                                                                            end: 2006,
                                                                            as_str(): "second_color",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2006,
                                                                end: 2007,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        path_seg: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 2007,
                                                                    end: 2010,
                                                                    as_str(): "rgb",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2010,
                                                                end: 2012,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2012,
                                                            end: 2013,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2018,
                                                            end: 2021,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2022,
                                                                end: 2034,
                                                                as_str(): "second_color",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2035,
                                                            end: 2036,
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
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 2037,
                                                                        end: 2049,
                                                                        as_str(): "PrimaryColor",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 2049,
                                                                            end: 2051,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                                ),
                                                                                start: 2051,
                                                                                end: 2055,
                                                                                as_str(): "Blue",
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
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2055,
                                                            end: 2056,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2061,
                                                            end: 2064,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2065,
                                                                end: 2075,
                                                                as_str(): "second_rgb",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2076,
                                                            end: 2077,
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
                                                                            src (ptr): 0x00007fe0fb117fe0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                            ),
                                                                            start: 2078,
                                                                            end: 2090,
                                                                            as_str(): "second_color",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2090,
                                                                end: 2091,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        path_seg: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 2091,
                                                                    end: 2094,
                                                                    as_str(): "rgb",
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
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 2094,
                                                                end: 2096,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2096,
                                                            end: 2097,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2102,
                                                            end: 2104,
                                                            as_str(): "10",
                                                        },
                                                        parsed: 10,
                                                        ty_opt: Some(
                                                            (
                                                                U32,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 2104,
                                                                    end: 2107,
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
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 1547,
                                        end: 2109,
                                        as_str(): "{\n    let first_color: PrimaryColor = PrimaryColor::Green;\n    let test = first_color == PrimaryColor::Green;\n    // Specifically, when we call methods in the below way, `self` is undefined\n    let rgb: Rgb = first_color.rgb();\n    // now, going to test the register pool by using over 48 registers\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    10u32\n}",
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
