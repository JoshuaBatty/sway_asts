Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe07c64be10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe07c64be10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
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
                                            src (ptr): 0x00007fe07c64be10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 15,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe07c64be10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                            ),
                                            start: 15,
                                            end: 17,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe07c64be10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                ),
                                                start: 17,
                                                end: 23,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe07c64be10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                ),
                                                start: 23,
                                                end: 25,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c64be10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                    ),
                                                    start: 25,
                                                    end: 31,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 31,
                                        end: 32,
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 34,
                                        end: 40,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 42,
                                        as_str(): "I",
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 49,
                                                                end: 53,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 54,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 55,
                                                                            end: 58,
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 58,
                                                        end: 59,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 64,
                                                            end: 67,
                                                            as_str(): "one",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 67,
                                                            end: 68,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 69,
                                                                        end: 71,
                                                                        as_str(): "u8",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 73,
                                        as_str(): "{\n    zero: u64,\n    one: u8\n}",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 75,
                                        end: 81,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 82,
                                        end: 84,
                                        as_str(): "T6",
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 91,
                                                                end: 95,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 96,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 97,
                                                                            end: 99,
                                                                            as_str(): "u8",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 99,
                                                        end: 100,
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 108,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 108,
                                                                end: 109,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 110,
                                                                            end: 112,
                                                                            as_str(): "u8",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 112,
                                                        end: 113,
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 121,
                                                                as_str(): "two",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 121,
                                                                end: 122,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 123,
                                                                            end: 124,
                                                                            as_str(): "I",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 124,
                                                        end: 125,
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 130,
                                                                end: 135,
                                                                as_str(): "three",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 135,
                                                                end: 136,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 137,
                                                                            end: 140,
                                                                            as_str(): "u16",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 140,
                                                        end: 141,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 150,
                                                            as_str(): "four",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 150,
                                                            end: 151,
                                                            as_str(): ":",
                                                        },
                                                    },
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 153,
                                                                                    end: 155,
                                                                                    as_str(): "u8",
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 155,
                                                                        end: 156,
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
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 157,
                                                                                            end: 159,
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
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 152,
                                                                end: 160,
                                                                as_str(): "(u8, u8)",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 85,
                                        end: 162,
                                        as_str(): "{\n    zero: u8,\n    one: u8,\n    two: I,\n    three: u16,\n    four: (u8, u8)\n}",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 164,
                                        end: 170,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 171,
                                        end: 172,
                                        as_str(): "W",
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 181,
                                                                as_str(): "t5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 181,
                                                                end: 182,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 183,
                                                                            end: 186,
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 186,
                                                        end: 187,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 192,
                                                            end: 194,
                                                            as_str(): "t6",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 195,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 196,
                                                                        end: 198,
                                                                        as_str(): "T6",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 173,
                                        end: 200,
                                        as_str(): "{\n    t5: u64,\n    t6: T6\n}",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 202,
                                        end: 208,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 209,
                                        end: 211,
                                        as_str(): "T3",
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 218,
                                                                end: 222,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 222,
                                                                end: 223,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 224,
                                                                            end: 226,
                                                                            as_str(): "u8",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 226,
                                                        end: 227,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 232,
                                                            end: 235,
                                                            as_str(): "one",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 235,
                                                            end: 236,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 237,
                                                                        end: 239,
                                                                        as_str(): "u8",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 212,
                                        end: 241,
                                        as_str(): "{\n    zero: u8,\n    one: u8\n}",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 243,
                                        end: 249,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 250,
                                        end: 251,
                                        as_str(): "T",
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 258,
                                                                end: 260,
                                                                as_str(): "t3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 260,
                                                                end: 261,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 262,
                                                                            end: 264,
                                                                            as_str(): "T3",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 264,
                                                        end: 265,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 272,
                                                            as_str(): "t4",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 272,
                                                            end: 273,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 274,
                                                                        end: 277,
                                                                        as_str(): "u16",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 252,
                                        end: 279,
                                        as_str(): "{\n    t3: T3,\n    t4: u16\n}",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 281,
                                        end: 287,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 288,
                                        end: 290,
                                        as_str(): "T1",
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 297,
                                                                end: 301,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 301,
                                                                end: 302,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 306,
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 306,
                                                        end: 307,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 312,
                                                            end: 315,
                                                            as_str(): "one",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 315,
                                                            end: 316,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 317,
                                                                        end: 320,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 291,
                                        end: 322,
                                        as_str(): "{\n    zero: u64,\n    one: u64\n}",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 324,
                                        end: 330,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 331,
                                        end: 332,
                                        as_str(): "S",
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 339,
                                                                end: 341,
                                                                as_str(): "t0",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 341,
                                                                end: 342,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 343,
                                                                            end: 344,
                                                                            as_str(): "W",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 344,
                                                        end: 345,
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 350,
                                                                end: 352,
                                                                as_str(): "t1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 352,
                                                                end: 353,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 354,
                                                                            end: 356,
                                                                            as_str(): "T1",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 356,
                                                        end: 357,
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 362,
                                                                end: 364,
                                                                as_str(): "t2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 364,
                                                                end: 365,
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
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 366,
                                                                            end: 367,
                                                                            as_str(): "T",
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
                                                        src (ptr): 0x00007fe07c64be10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                        ),
                                                        start: 367,
                                                        end: 368,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 373,
                                                            end: 375,
                                                            as_str(): "t3",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 375,
                                                            end: 376,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 377,
                                                                        end: 380,
                                                                        as_str(): "u16",
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
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 333,
                                        end: 382,
                                        as_str(): "{\n    t0: W,\n    t1: T1,\n    t2: T,\n    t3: u16\n}",
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
                                            src (ptr): 0x00007fe07c64be10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                            ),
                                            start: 384,
                                            end: 386,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe07c64be10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                            ),
                                            start: 387,
                                            end: 391,
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
                                            src (ptr): 0x00007fe07c64be10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                            ),
                                            start: 391,
                                            end: 393,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe07c64be10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                    ),
                                                    start: 394,
                                                    end: 396,
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
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 397,
                                                                end: 401,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 408,
                                                            end: 411,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 412,
                                                                    end: 415,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 416,
                                                                end: 417,
                                                                as_str(): "s",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 418,
                                                            end: 419,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 420,
                                                                        end: 421,
                                                                        as_str(): "S",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 432,
                                                                                    end: 434,
                                                                                    as_str(): "t0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 434,
                                                                                            end: 435,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Struct {
                                                                                        path: PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 436,
                                                                                                        end: 437,
                                                                                                        as_str(): "W",
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
                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 452,
                                                                                                                    end: 454,
                                                                                                                    as_str(): "t5",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 454,
                                                                                                                            end: 455,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 456,
                                                                                                                                    end: 457,
                                                                                                                                    as_str(): "5",
                                                                                                                                },
                                                                                                                                parsed: 5,
                                                                                                                                ty_opt: None,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 457,
                                                                                                                end: 458,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 471,
                                                                                                                end: 473,
                                                                                                                as_str(): "t6",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 473,
                                                                                                                        end: 474,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Struct {
                                                                                                                    path: PathExpr {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 475,
                                                                                                                                    end: 477,
                                                                                                                                    as_str(): "T6",
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
                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 496,
                                                                                                                                                end: 500,
                                                                                                                                                as_str(): "zero",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 500,
                                                                                                                                                        end: 501,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 502,
                                                                                                                                                                end: 503,
                                                                                                                                                                as_str(): "6",
                                                                                                                                                            },
                                                                                                                                                            parsed: 6,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 503,
                                                                                                                                            end: 504,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                (
                                                                                                                                    ExprStructField {
                                                                                                                                        field_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 521,
                                                                                                                                                end: 524,
                                                                                                                                                as_str(): "one",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 524,
                                                                                                                                                        end: 525,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 526,
                                                                                                                                                                end: 527,
                                                                                                                                                                as_str(): "7",
                                                                                                                                                            },
                                                                                                                                                            parsed: 7,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 527,
                                                                                                                                            end: 528,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                (
                                                                                                                                    ExprStructField {
                                                                                                                                        field_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 546,
                                                                                                                                                end: 549,
                                                                                                                                                as_str(): "two",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 549,
                                                                                                                                                        end: 550,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Struct {
                                                                                                                                                    path: PathExpr {
                                                                                                                                                        root_opt: None,
                                                                                                                                                        prefix: PathExprSegment {
                                                                                                                                                            name: BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 551,
                                                                                                                                                                    end: 552,
                                                                                                                                                                    as_str(): "I",
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
                                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 575,
                                                                                                                                                                                end: 579,
                                                                                                                                                                                as_str(): "zero",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        expr_opt: Some(
                                                                                                                                                                            (
                                                                                                                                                                                ColonToken {
                                                                                                                                                                                    span: Span {
                                                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                                        path: Some(
                                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                                        ),
                                                                                                                                                                                        start: 579,
                                                                                                                                                                                        end: 580,
                                                                                                                                                                                        as_str(): ":",
                                                                                                                                                                                    },
                                                                                                                                                                                },
                                                                                                                                                                                Literal(
                                                                                                                                                                                    Int(
                                                                                                                                                                                        LitInt {
                                                                                                                                                                                            span: Span {
                                                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                                                path: Some(
                                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                                                ),
                                                                                                                                                                                                start: 581,
                                                                                                                                                                                                end: 582,
                                                                                                                                                                                                as_str(): "8",
                                                                                                                                                                                            },
                                                                                                                                                                                            parsed: 8,
                                                                                                                                                                                            ty_opt: None,
                                                                                                                                                                                        },
                                                                                                                                                                                    ),
                                                                                                                                                                                ),
                                                                                                                                                                            ),
                                                                                                                                                                        ),
                                                                                                                                                                    },
                                                                                                                                                                    CommaToken {
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 582,
                                                                                                                                                                            end: 583,
                                                                                                                                                                            as_str(): ",",
                                                                                                                                                                        },
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                            ],
                                                                                                                                                            final_value_opt: Some(
                                                                                                                                                                ExprStructField {
                                                                                                                                                                    field_name: BaseIdent {
                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 605,
                                                                                                                                                                            end: 608,
                                                                                                                                                                            as_str(): "one",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                    expr_opt: Some(
                                                                                                                                                                        (
                                                                                                                                                                            ColonToken {
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 608,
                                                                                                                                                                                    end: 609,
                                                                                                                                                                                    as_str(): ":",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            Literal(
                                                                                                                                                                                Int(
                                                                                                                                                                                    LitInt {
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 610,
                                                                                                                                                                                            end: 611,
                                                                                                                                                                                            as_str(): "9",
                                                                                                                                                                                        },
                                                                                                                                                                                        parsed: 9,
                                                                                                                                                                                        ty_opt: None,
                                                                                                                                                                                    },
                                                                                                                                                                                ),
                                                                                                                                                                            ),
                                                                                                                                                                        ),
                                                                                                                                                                    ),
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                        },
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 553,
                                                                                                                                                            end: 629,
                                                                                                                                                            as_str(): "{\n                    zero: 8, \n                    one: 9\n                }",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 629,
                                                                                                                                            end: 630,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                (
                                                                                                                                    ExprStructField {
                                                                                                                                        field_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 647,
                                                                                                                                                end: 652,
                                                                                                                                                as_str(): "three",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 652,
                                                                                                                                                        end: 653,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 654,
                                                                                                                                                                end: 656,
                                                                                                                                                                as_str(): "10",
                                                                                                                                                            },
                                                                                                                                                            parsed: 10,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 656,
                                                                                                                                            end: 657,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ],
                                                                                                                            final_value_opt: Some(
                                                                                                                                ExprStructField {
                                                                                                                                    field_name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 674,
                                                                                                                                            end: 678,
                                                                                                                                            as_str(): "four",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    expr_opt: Some(
                                                                                                                                        (
                                                                                                                                            ColonToken {
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 678,
                                                                                                                                                    end: 679,
                                                                                                                                                    as_str(): ":",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                            Tuple(
                                                                                                                                                Parens {
                                                                                                                                                    inner: Cons {
                                                                                                                                                        head: Literal(
                                                                                                                                                            Int(
                                                                                                                                                                LitInt {
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 681,
                                                                                                                                                                        end: 683,
                                                                                                                                                                        as_str(): "11",
                                                                                                                                                                    },
                                                                                                                                                                    parsed: 11,
                                                                                                                                                                    ty_opt: None,
                                                                                                                                                                },
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                        comma_token: CommaToken {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 683,
                                                                                                                                                                end: 684,
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
                                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 685,
                                                                                                                                                                                end: 687,
                                                                                                                                                                                as_str(): "12",
                                                                                                                                                                            },
                                                                                                                                                                            parsed: 12,
                                                                                                                                                                            ty_opt: None,
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 680,
                                                                                                                                                        end: 688,
                                                                                                                                                        as_str(): "(11, 12)",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 478,
                                                                                                                            end: 702,
                                                                                                                            as_str(): "{\n                zero: 6,\n                one: 7, \n                two: I {\n                    zero: 8, \n                    one: 9\n                },\n                three: 10,\n                four: (11, 12)\n            }",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 438,
                                                                                                end: 712,
                                                                                                as_str(): "{\n            t5: 5,\n            t6: T6 {\n                zero: 6,\n                one: 7, \n                two: I {\n                    zero: 8, \n                    one: 9\n                },\n                three: 10,\n                four: (11, 12)\n            }\n        }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 712,
                                                                                end: 713,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 722,
                                                                                    end: 724,
                                                                                    as_str(): "t1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 724,
                                                                                            end: 725,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Struct {
                                                                                        path: PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 726,
                                                                                                        end: 728,
                                                                                                        as_str(): "T1",
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
                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 743,
                                                                                                                    end: 747,
                                                                                                                    as_str(): "zero",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 747,
                                                                                                                            end: 748,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 749,
                                                                                                                                    end: 750,
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
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 750,
                                                                                                                end: 751,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 765,
                                                                                                                end: 768,
                                                                                                                as_str(): "one",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 768,
                                                                                                                        end: 769,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 770,
                                                                                                                                end: 771,
                                                                                                                                as_str(): "1",
                                                                                                                            },
                                                                                                                            parsed: 1,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 729,
                                                                                                end: 781,
                                                                                                as_str(): "{\n            zero: 0, \n            one: 1\n        }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 781,
                                                                                end: 782,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 791,
                                                                                    end: 793,
                                                                                    as_str(): "t2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 793,
                                                                                            end: 794,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Struct {
                                                                                        path: PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 795,
                                                                                                        end: 796,
                                                                                                        as_str(): "T",
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
                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 811,
                                                                                                                    end: 813,
                                                                                                                    as_str(): "t3",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 813,
                                                                                                                            end: 814,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Struct {
                                                                                                                        path: PathExpr {
                                                                                                                            root_opt: None,
                                                                                                                            prefix: PathExprSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 815,
                                                                                                                                        end: 817,
                                                                                                                                        as_str(): "T3",
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
                                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 836,
                                                                                                                                                    end: 840,
                                                                                                                                                    as_str(): "zero",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            expr_opt: Some(
                                                                                                                                                (
                                                                                                                                                    ColonToken {
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 840,
                                                                                                                                                            end: 841,
                                                                                                                                                            as_str(): ":",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    Literal(
                                                                                                                                                        Int(
                                                                                                                                                            LitInt {
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 842,
                                                                                                                                                                    end: 843,
                                                                                                                                                                    as_str(): "2",
                                                                                                                                                                },
                                                                                                                                                                parsed: 2,
                                                                                                                                                                ty_opt: None,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        },
                                                                                                                                        CommaToken {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 843,
                                                                                                                                                end: 844,
                                                                                                                                                as_str(): ",",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ],
                                                                                                                                final_value_opt: Some(
                                                                                                                                    ExprStructField {
                                                                                                                                        field_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 862,
                                                                                                                                                end: 865,
                                                                                                                                                as_str(): "one",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        expr_opt: Some(
                                                                                                                                            (
                                                                                                                                                ColonToken {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 865,
                                                                                                                                                        end: 866,
                                                                                                                                                        as_str(): ":",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                Literal(
                                                                                                                                                    Int(
                                                                                                                                                        LitInt {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 867,
                                                                                                                                                                end: 868,
                                                                                                                                                                as_str(): "3",
                                                                                                                                                            },
                                                                                                                                                            parsed: 3,
                                                                                                                                                            ty_opt: None,
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 818,
                                                                                                                                end: 882,
                                                                                                                                as_str(): "{\n                zero: 2, \n                one: 3\n            }",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 882,
                                                                                                                end: 883,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 896,
                                                                                                                end: 898,
                                                                                                                as_str(): "t4",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 898,
                                                                                                                        end: 899,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 900,
                                                                                                                                end: 901,
                                                                                                                                as_str(): "4",
                                                                                                                            },
                                                                                                                            parsed: 4,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 797,
                                                                                                end: 911,
                                                                                                as_str(): "{\n            t3: T3 {\n                zero: 2, \n                one: 3\n            },\n            t4: 4\n        }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 911,
                                                                                end: 912,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    ExprStructField {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 921,
                                                                                end: 923,
                                                                                as_str(): "t3",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 923,
                                                                                        end: 924,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 925,
                                                                                                end: 927,
                                                                                                as_str(): "13",
                                                                                            },
                                                                                            parsed: 13,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 422,
                                                                end: 933,
                                                                as_str(): "{\n        t0: W {\n            t5: 5,\n            t6: T6 {\n                zero: 6,\n                one: 7, \n                two: I {\n                    zero: 8, \n                    one: 9\n                },\n                three: 10,\n                four: (11, 12)\n            }\n        },\n        t1: T1 {\n            zero: 0, \n            one: 1\n        },\n        t2: T {\n            t3: T3 {\n                zero: 2, \n                one: 3\n            },\n            t4: 4\n        },\n        t3: 13\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 933,
                                                            end: 934,
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
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 940,
                                                                        end: 946,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 947,
                                                                                                end: 948,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 948,
                                                                                    end: 949,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 949,
                                                                                    end: 951,
                                                                                    as_str(): "t1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 951,
                                                                                end: 952,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 952,
                                                                                end: 956,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 957,
                                                                            end: 959,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 960,
                                                                                    end: 961,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 946,
                                                            end: 962,
                                                            as_str(): "(s.t1.zero == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 962,
                                                            end: 963,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 968,
                                                                        end: 974,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 975,
                                                                                                end: 976,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 976,
                                                                                    end: 977,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 977,
                                                                                    end: 979,
                                                                                    as_str(): "t1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 979,
                                                                                end: 980,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 980,
                                                                                end: 983,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 984,
                                                                            end: 986,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 987,
                                                                                    end: 988,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 974,
                                                            end: 989,
                                                            as_str(): "(s.t1.one == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 989,
                                                            end: 990,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 995,
                                                                        end: 1001,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1002,
                                                                                                    end: 1003,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1003,
                                                                                        end: 1004,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1004,
                                                                                        end: 1006,
                                                                                        as_str(): "t2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1006,
                                                                                    end: 1007,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1007,
                                                                                    end: 1009,
                                                                                    as_str(): "t3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1009,
                                                                                end: 1010,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1010,
                                                                                end: 1014,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1015,
                                                                            end: 1017,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1018,
                                                                                    end: 1019,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1001,
                                                            end: 1020,
                                                            as_str(): "(s.t2.t3.zero == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1020,
                                                            end: 1021,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1026,
                                                                        end: 1032,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1033,
                                                                                                    end: 1034,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1034,
                                                                                        end: 1035,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1035,
                                                                                        end: 1037,
                                                                                        as_str(): "t2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1037,
                                                                                    end: 1038,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1038,
                                                                                    end: 1040,
                                                                                    as_str(): "t3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1040,
                                                                                end: 1041,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1041,
                                                                                end: 1044,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1045,
                                                                            end: 1047,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1048,
                                                                                    end: 1049,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1032,
                                                            end: 1050,
                                                            as_str(): "(s.t2.t3.one == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1050,
                                                            end: 1051,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1056,
                                                                        end: 1062,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1063,
                                                                                                end: 1064,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1064,
                                                                                    end: 1065,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1065,
                                                                                    end: 1067,
                                                                                    as_str(): "t2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1067,
                                                                                end: 1068,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1068,
                                                                                end: 1070,
                                                                                as_str(): "t4",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1071,
                                                                            end: 1073,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1074,
                                                                                    end: 1075,
                                                                                    as_str(): "4",
                                                                                },
                                                                                parsed: 4,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1062,
                                                            end: 1076,
                                                            as_str(): "(s.t2.t4 == 4)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1076,
                                                            end: 1077,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1082,
                                                                        end: 1088,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1089,
                                                                                                end: 1090,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1090,
                                                                                    end: 1091,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1091,
                                                                                    end: 1093,
                                                                                    as_str(): "t0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1093,
                                                                                end: 1094,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1094,
                                                                                end: 1096,
                                                                                as_str(): "t5",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1097,
                                                                            end: 1099,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1100,
                                                                                    end: 1101,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1088,
                                                            end: 1102,
                                                            as_str(): "(s.t0.t5 == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1102,
                                                            end: 1103,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1108,
                                                                        end: 1114,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1115,
                                                                                                    end: 1116,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1116,
                                                                                        end: 1117,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1117,
                                                                                        end: 1119,
                                                                                        as_str(): "t0",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1119,
                                                                                    end: 1120,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1120,
                                                                                    end: 1122,
                                                                                    as_str(): "t6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1122,
                                                                                end: 1123,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1123,
                                                                                end: 1127,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1128,
                                                                            end: 1130,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1131,
                                                                                    end: 1132,
                                                                                    as_str(): "6",
                                                                                },
                                                                                parsed: 6,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1114,
                                                            end: 1133,
                                                            as_str(): "(s.t0.t6.zero == 6)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1133,
                                                            end: 1134,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1139,
                                                                        end: 1145,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1146,
                                                                                                    end: 1147,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1147,
                                                                                        end: 1148,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1148,
                                                                                        end: 1150,
                                                                                        as_str(): "t0",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1150,
                                                                                    end: 1151,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1151,
                                                                                    end: 1153,
                                                                                    as_str(): "t6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1153,
                                                                                end: 1154,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1154,
                                                                                end: 1157,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1158,
                                                                            end: 1160,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1161,
                                                                                    end: 1162,
                                                                                    as_str(): "7",
                                                                                },
                                                                                parsed: 7,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1145,
                                                            end: 1163,
                                                            as_str(): "(s.t0.t6.one == 7)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1163,
                                                            end: 1164,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1169,
                                                                        end: 1175,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: FieldProjection {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1176,
                                                                                                        end: 1177,
                                                                                                        as_str(): "s",
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
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1177,
                                                                                            end: 1178,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1178,
                                                                                            end: 1180,
                                                                                            as_str(): "t0",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                dot_token: DotToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1180,
                                                                                        end: 1181,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1181,
                                                                                        end: 1183,
                                                                                        as_str(): "t6",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1183,
                                                                                    end: 1184,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1184,
                                                                                    end: 1187,
                                                                                    as_str(): "two",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1187,
                                                                                end: 1188,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1188,
                                                                                end: 1192,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1193,
                                                                            end: 1195,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1196,
                                                                                    end: 1197,
                                                                                    as_str(): "8",
                                                                                },
                                                                                parsed: 8,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1175,
                                                            end: 1198,
                                                            as_str(): "(s.t0.t6.two.zero == 8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1198,
                                                            end: 1199,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1204,
                                                                        end: 1210,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: FieldProjection {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1211,
                                                                                                        end: 1212,
                                                                                                        as_str(): "s",
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
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1212,
                                                                                            end: 1213,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1213,
                                                                                            end: 1215,
                                                                                            as_str(): "t0",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                dot_token: DotToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1215,
                                                                                        end: 1216,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1216,
                                                                                        end: 1218,
                                                                                        as_str(): "t6",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1218,
                                                                                    end: 1219,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1219,
                                                                                    end: 1222,
                                                                                    as_str(): "two",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1222,
                                                                                end: 1223,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1223,
                                                                                end: 1226,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1227,
                                                                            end: 1229,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1230,
                                                                                    end: 1231,
                                                                                    as_str(): "9",
                                                                                },
                                                                                parsed: 9,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1210,
                                                            end: 1232,
                                                            as_str(): "(s.t0.t6.two.one == 9)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1232,
                                                            end: 1233,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1238,
                                                                        end: 1244,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1245,
                                                                                                    end: 1246,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1246,
                                                                                        end: 1247,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1247,
                                                                                        end: 1249,
                                                                                        as_str(): "t0",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1249,
                                                                                    end: 1250,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1250,
                                                                                    end: 1252,
                                                                                    as_str(): "t6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1252,
                                                                                end: 1253,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1253,
                                                                                end: 1258,
                                                                                as_str(): "three",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1259,
                                                                            end: 1261,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1262,
                                                                                    end: 1264,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1244,
                                                            end: 1265,
                                                            as_str(): "(s.t0.t6.three == 10)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1265,
                                                            end: 1266,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1271,
                                                                        end: 1277,
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
                                                                    lhs: TupleFieldProjection {
                                                                        target: Parens(
                                                                            Parens {
                                                                                inner: FieldProjection {
                                                                                    target: FieldProjection {
                                                                                        target: FieldProjection {
                                                                                            target: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1279,
                                                                                                                end: 1280,
                                                                                                                as_str(): "s",
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
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1280,
                                                                                                    end: 1281,
                                                                                                    as_str(): ".",
                                                                                                },
                                                                                            },
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1281,
                                                                                                    end: 1283,
                                                                                                    as_str(): "t0",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        dot_token: DotToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1283,
                                                                                                end: 1284,
                                                                                                as_str(): ".",
                                                                                            },
                                                                                        },
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1284,
                                                                                                end: 1286,
                                                                                                as_str(): "t6",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1286,
                                                                                            end: 1287,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1287,
                                                                                            end: 1291,
                                                                                            as_str(): "four",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1278,
                                                                                    end: 1292,
                                                                                    as_str(): "(s.t0.t6.four)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1292,
                                                                                end: 1293,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1293,
                                                                            end: 1294,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1295,
                                                                            end: 1297,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1298,
                                                                                    end: 1300,
                                                                                    as_str(): "11",
                                                                                },
                                                                                parsed: 11,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1277,
                                                            end: 1301,
                                                            as_str(): "((s.t0.t6.four).0 == 11)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1301,
                                                            end: 1302,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1307,
                                                                        end: 1313,
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
                                                                    lhs: TupleFieldProjection {
                                                                        target: Parens(
                                                                            Parens {
                                                                                inner: FieldProjection {
                                                                                    target: FieldProjection {
                                                                                        target: FieldProjection {
                                                                                            target: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1315,
                                                                                                                end: 1316,
                                                                                                                as_str(): "s",
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
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1316,
                                                                                                    end: 1317,
                                                                                                    as_str(): ".",
                                                                                                },
                                                                                            },
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1317,
                                                                                                    end: 1319,
                                                                                                    as_str(): "t0",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        dot_token: DotToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1319,
                                                                                                end: 1320,
                                                                                                as_str(): ".",
                                                                                            },
                                                                                        },
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1320,
                                                                                                end: 1322,
                                                                                                as_str(): "t6",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1322,
                                                                                            end: 1323,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1323,
                                                                                            end: 1327,
                                                                                            as_str(): "four",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1314,
                                                                                    end: 1328,
                                                                                    as_str(): "(s.t0.t6.four)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1328,
                                                                                end: 1329,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1329,
                                                                            end: 1330,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1334,
                                                                                    end: 1336,
                                                                                    as_str(): "12",
                                                                                },
                                                                                parsed: 12,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1313,
                                                            end: 1337,
                                                            as_str(): "((s.t0.t6.four).1 == 12)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1337,
                                                            end: 1338,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1343,
                                                                        end: 1349,
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
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1350,
                                                                                            end: 1351,
                                                                                            as_str(): "s",
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
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1351,
                                                                                end: 1352,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1352,
                                                                                end: 1354,
                                                                                as_str(): "t3",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1355,
                                                                            end: 1357,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1358,
                                                                                    end: 1360,
                                                                                    as_str(): "13",
                                                                                },
                                                                                parsed: 13,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1349,
                                                            end: 1361,
                                                            as_str(): "(s.t3 == 13)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1361,
                                                            end: 1362,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: Var(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1368,
                                                                        end: 1369,
                                                                        as_str(): "s",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1369,
                                                                    end: 1370,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1370,
                                                                    end: 1372,
                                                                    as_str(): "t1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1372,
                                                                end: 1373,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1373,
                                                                end: 1377,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1378,
                                                            end: 1379,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1380,
                                                                    end: 1382,
                                                                    as_str(): "10",
                                                                },
                                                                parsed: 10,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1382,
                                                            end: 1383,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: Var(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1388,
                                                                        end: 1389,
                                                                        as_str(): "s",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1389,
                                                                    end: 1390,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1390,
                                                                    end: 1392,
                                                                    as_str(): "t1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1392,
                                                                end: 1393,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1393,
                                                                end: 1396,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1397,
                                                            end: 1398,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1399,
                                                                    end: 1401,
                                                                    as_str(): "11",
                                                                },
                                                                parsed: 11,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1401,
                                                            end: 1402,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1407,
                                                                            end: 1408,
                                                                            as_str(): "s",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1408,
                                                                        end: 1409,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1409,
                                                                        end: 1411,
                                                                        as_str(): "t2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1411,
                                                                    end: 1412,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1412,
                                                                    end: 1414,
                                                                    as_str(): "t3",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1414,
                                                                end: 1415,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1415,
                                                                end: 1419,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1420,
                                                            end: 1421,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1422,
                                                                    end: 1424,
                                                                    as_str(): "12",
                                                                },
                                                                parsed: 12,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1424,
                                                            end: 1425,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1430,
                                                                            end: 1431,
                                                                            as_str(): "s",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1431,
                                                                        end: 1432,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1432,
                                                                        end: 1434,
                                                                        as_str(): "t2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1434,
                                                                    end: 1435,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1435,
                                                                    end: 1437,
                                                                    as_str(): "t3",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1437,
                                                                end: 1438,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1438,
                                                                end: 1441,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1442,
                                                            end: 1443,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1444,
                                                                    end: 1446,
                                                                    as_str(): "13",
                                                                },
                                                                parsed: 13,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1446,
                                                            end: 1447,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: Var(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1452,
                                                                        end: 1453,
                                                                        as_str(): "s",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1453,
                                                                    end: 1454,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1454,
                                                                    end: 1456,
                                                                    as_str(): "t2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1456,
                                                                end: 1457,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1457,
                                                                end: 1459,
                                                                as_str(): "t4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1460,
                                                            end: 1461,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1462,
                                                                    end: 1464,
                                                                    as_str(): "14",
                                                                },
                                                                parsed: 14,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1464,
                                                            end: 1465,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: Var(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1470,
                                                                        end: 1471,
                                                                        as_str(): "s",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1471,
                                                                    end: 1472,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1472,
                                                                    end: 1474,
                                                                    as_str(): "t0",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1474,
                                                                end: 1475,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1475,
                                                                end: 1477,
                                                                as_str(): "t5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1478,
                                                            end: 1479,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1480,
                                                                    end: 1482,
                                                                    as_str(): "15",
                                                                },
                                                                parsed: 15,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1482,
                                                            end: 1483,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1488,
                                                                            end: 1489,
                                                                            as_str(): "s",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1489,
                                                                        end: 1490,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1490,
                                                                        end: 1492,
                                                                        as_str(): "t0",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1492,
                                                                    end: 1493,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1493,
                                                                    end: 1495,
                                                                    as_str(): "t6",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1495,
                                                                end: 1496,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1496,
                                                                end: 1500,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1501,
                                                            end: 1502,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1503,
                                                                    end: 1505,
                                                                    as_str(): "16",
                                                                },
                                                                parsed: 16,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1505,
                                                            end: 1506,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1511,
                                                                            end: 1512,
                                                                            as_str(): "s",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1512,
                                                                        end: 1513,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1513,
                                                                        end: 1515,
                                                                        as_str(): "t0",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1515,
                                                                    end: 1516,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1516,
                                                                    end: 1518,
                                                                    as_str(): "t6",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1518,
                                                                end: 1519,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1519,
                                                                end: 1522,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1523,
                                                            end: 1524,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1525,
                                                                    end: 1527,
                                                                    as_str(): "17",
                                                                },
                                                                parsed: 17,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1527,
                                                            end: 1528,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: FieldProjection {
                                                                    target: Var(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1533,
                                                                                end: 1534,
                                                                                as_str(): "s",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1534,
                                                                            end: 1535,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1535,
                                                                            end: 1537,
                                                                            as_str(): "t0",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1537,
                                                                        end: 1538,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1538,
                                                                        end: 1540,
                                                                        as_str(): "t6",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1540,
                                                                    end: 1541,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1541,
                                                                    end: 1544,
                                                                    as_str(): "two",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1544,
                                                                end: 1545,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1545,
                                                                end: 1549,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1550,
                                                            end: 1551,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1552,
                                                                    end: 1554,
                                                                    as_str(): "18",
                                                                },
                                                                parsed: 18,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1554,
                                                            end: 1555,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: FieldProjection {
                                                                    target: Var(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1560,
                                                                                end: 1561,
                                                                                as_str(): "s",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1561,
                                                                            end: 1562,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1562,
                                                                            end: 1564,
                                                                            as_str(): "t0",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1564,
                                                                        end: 1565,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1565,
                                                                        end: 1567,
                                                                        as_str(): "t6",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1567,
                                                                    end: 1568,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1568,
                                                                    end: 1571,
                                                                    as_str(): "two",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1571,
                                                                end: 1572,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1572,
                                                                end: 1575,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1576,
                                                            end: 1577,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1578,
                                                                    end: 1580,
                                                                    as_str(): "19",
                                                                },
                                                                parsed: 19,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1580,
                                                            end: 1581,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1586,
                                                                            end: 1587,
                                                                            as_str(): "s",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1587,
                                                                        end: 1588,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1588,
                                                                        end: 1590,
                                                                        as_str(): "t0",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1590,
                                                                    end: 1591,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1591,
                                                                    end: 1593,
                                                                    as_str(): "t6",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1593,
                                                                end: 1594,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1594,
                                                                end: 1599,
                                                                as_str(): "three",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1600,
                                                            end: 1601,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1602,
                                                                    end: 1605,
                                                                    as_str(): "110",
                                                                },
                                                                parsed: 110,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1605,
                                                            end: 1606,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: FieldProjection {
                                                            target: FieldProjection {
                                                                target: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1611,
                                                                            end: 1612,
                                                                            as_str(): "s",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1612,
                                                                        end: 1613,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1613,
                                                                        end: 1615,
                                                                        as_str(): "t0",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1615,
                                                                    end: 1616,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1616,
                                                                    end: 1618,
                                                                    as_str(): "t6",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1618,
                                                                end: 1619,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1619,
                                                                end: 1623,
                                                                as_str(): "four",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1624,
                                                            end: 1625,
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
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1627,
                                                                                end: 1630,
                                                                                as_str(): "111",
                                                                            },
                                                                            parsed: 111,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1630,
                                                                        end: 1631,
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1632,
                                                                                        end: 1635,
                                                                                        as_str(): "112",
                                                                                    },
                                                                                    parsed: 112,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1626,
                                                                end: 1636,
                                                                as_str(): "(111, 112)",
                                                            },
                                                        },
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1636,
                                                            end: 1637,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: Var(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1642,
                                                                    end: 1643,
                                                                    as_str(): "s",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1643,
                                                                end: 1644,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c64be10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                ),
                                                                start: 1644,
                                                                end: 1646,
                                                                as_str(): "t3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1647,
                                                            end: 1648,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c64be10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                    ),
                                                                    start: 1649,
                                                                    end: 1652,
                                                                    as_str(): "113",
                                                                },
                                                                parsed: 113,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1652,
                                                            end: 1653,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1659,
                                                                        end: 1665,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1666,
                                                                                                end: 1667,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1667,
                                                                                    end: 1668,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1668,
                                                                                    end: 1670,
                                                                                    as_str(): "t1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1670,
                                                                                end: 1671,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1671,
                                                                                end: 1675,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1676,
                                                                            end: 1678,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1679,
                                                                                    end: 1681,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1665,
                                                            end: 1682,
                                                            as_str(): "(s.t1.zero == 10)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1682,
                                                            end: 1683,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1688,
                                                                        end: 1694,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1695,
                                                                                                end: 1696,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1696,
                                                                                    end: 1697,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1697,
                                                                                    end: 1699,
                                                                                    as_str(): "t1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1699,
                                                                                end: 1700,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1700,
                                                                                end: 1703,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1704,
                                                                            end: 1706,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1707,
                                                                                    end: 1709,
                                                                                    as_str(): "11",
                                                                                },
                                                                                parsed: 11,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1694,
                                                            end: 1710,
                                                            as_str(): "(s.t1.one == 11)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1710,
                                                            end: 1711,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1716,
                                                                        end: 1722,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1723,
                                                                                                    end: 1724,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1724,
                                                                                        end: 1725,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1725,
                                                                                        end: 1727,
                                                                                        as_str(): "t2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1727,
                                                                                    end: 1728,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1728,
                                                                                    end: 1730,
                                                                                    as_str(): "t3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1730,
                                                                                end: 1731,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1731,
                                                                                end: 1735,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1736,
                                                                            end: 1738,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1739,
                                                                                    end: 1741,
                                                                                    as_str(): "12",
                                                                                },
                                                                                parsed: 12,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1722,
                                                            end: 1742,
                                                            as_str(): "(s.t2.t3.zero == 12)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1742,
                                                            end: 1743,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1748,
                                                                        end: 1754,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1755,
                                                                                                    end: 1756,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1756,
                                                                                        end: 1757,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1757,
                                                                                        end: 1759,
                                                                                        as_str(): "t2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1759,
                                                                                    end: 1760,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1760,
                                                                                    end: 1762,
                                                                                    as_str(): "t3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1762,
                                                                                end: 1763,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1763,
                                                                                end: 1766,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1767,
                                                                            end: 1769,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1770,
                                                                                    end: 1772,
                                                                                    as_str(): "13",
                                                                                },
                                                                                parsed: 13,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1754,
                                                            end: 1773,
                                                            as_str(): "(s.t2.t3.one == 13)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1773,
                                                            end: 1774,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1779,
                                                                        end: 1785,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1786,
                                                                                                end: 1787,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1787,
                                                                                    end: 1788,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1788,
                                                                                    end: 1790,
                                                                                    as_str(): "t2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1790,
                                                                                end: 1791,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1791,
                                                                                end: 1793,
                                                                                as_str(): "t4",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1794,
                                                                            end: 1796,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1797,
                                                                                    end: 1799,
                                                                                    as_str(): "14",
                                                                                },
                                                                                parsed: 14,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1785,
                                                            end: 1800,
                                                            as_str(): "(s.t2.t4 == 14)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1800,
                                                            end: 1801,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1806,
                                                                        end: 1812,
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
                                                                        target: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 1813,
                                                                                                end: 1814,
                                                                                                as_str(): "s",
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
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1814,
                                                                                    end: 1815,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1815,
                                                                                    end: 1817,
                                                                                    as_str(): "t0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1817,
                                                                                end: 1818,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1818,
                                                                                end: 1820,
                                                                                as_str(): "t5",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1821,
                                                                            end: 1823,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1824,
                                                                                    end: 1826,
                                                                                    as_str(): "15",
                                                                                },
                                                                                parsed: 15,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1812,
                                                            end: 1827,
                                                            as_str(): "(s.t0.t5 == 15)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1827,
                                                            end: 1828,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1833,
                                                                        end: 1839,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1840,
                                                                                                    end: 1841,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1841,
                                                                                        end: 1842,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1842,
                                                                                        end: 1844,
                                                                                        as_str(): "t0",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1844,
                                                                                    end: 1845,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1845,
                                                                                    end: 1847,
                                                                                    as_str(): "t6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1847,
                                                                                end: 1848,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1848,
                                                                                end: 1852,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1853,
                                                                            end: 1855,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1856,
                                                                                    end: 1858,
                                                                                    as_str(): "16",
                                                                                },
                                                                                parsed: 16,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1839,
                                                            end: 1859,
                                                            as_str(): "(s.t0.t6.zero == 16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1859,
                                                            end: 1860,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1865,
                                                                        end: 1871,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1872,
                                                                                                    end: 1873,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1873,
                                                                                        end: 1874,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1874,
                                                                                        end: 1876,
                                                                                        as_str(): "t0",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1876,
                                                                                    end: 1877,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1877,
                                                                                    end: 1879,
                                                                                    as_str(): "t6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1879,
                                                                                end: 1880,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1880,
                                                                                end: 1883,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1884,
                                                                            end: 1886,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1887,
                                                                                    end: 1889,
                                                                                    as_str(): "17",
                                                                                },
                                                                                parsed: 17,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1871,
                                                            end: 1890,
                                                            as_str(): "(s.t0.t6.one == 17)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1890,
                                                            end: 1891,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1896,
                                                                        end: 1902,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: FieldProjection {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1903,
                                                                                                        end: 1904,
                                                                                                        as_str(): "s",
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
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1904,
                                                                                            end: 1905,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1905,
                                                                                            end: 1907,
                                                                                            as_str(): "t0",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                dot_token: DotToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1907,
                                                                                        end: 1908,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1908,
                                                                                        end: 1910,
                                                                                        as_str(): "t6",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1910,
                                                                                    end: 1911,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1911,
                                                                                    end: 1914,
                                                                                    as_str(): "two",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1914,
                                                                                end: 1915,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1915,
                                                                                end: 1919,
                                                                                as_str(): "zero",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1920,
                                                                            end: 1922,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1923,
                                                                                    end: 1925,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1902,
                                                            end: 1926,
                                                            as_str(): "(s.t0.t6.two.zero == 18)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1926,
                                                            end: 1927,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1932,
                                                                        end: 1938,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: FieldProjection {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1939,
                                                                                                        end: 1940,
                                                                                                        as_str(): "s",
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
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1940,
                                                                                            end: 1941,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 1941,
                                                                                            end: 1943,
                                                                                            as_str(): "t0",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                dot_token: DotToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1943,
                                                                                        end: 1944,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1944,
                                                                                        end: 1946,
                                                                                        as_str(): "t6",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1946,
                                                                                    end: 1947,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1947,
                                                                                    end: 1950,
                                                                                    as_str(): "two",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1950,
                                                                                end: 1951,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1951,
                                                                                end: 1954,
                                                                                as_str(): "one",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1955,
                                                                            end: 1957,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1958,
                                                                                    end: 1960,
                                                                                    as_str(): "19",
                                                                                },
                                                                                parsed: 19,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1938,
                                                            end: 1961,
                                                            as_str(): "(s.t0.t6.two.one == 19)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1961,
                                                            end: 1962,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 1967,
                                                                        end: 1973,
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
                                                                        target: FieldProjection {
                                                                            target: FieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1974,
                                                                                                    end: 1975,
                                                                                                    as_str(): "s",
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
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1975,
                                                                                        end: 1976,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c64be10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                        ),
                                                                                        start: 1976,
                                                                                        end: 1978,
                                                                                        as_str(): "t0",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1978,
                                                                                    end: 1979,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1979,
                                                                                    end: 1981,
                                                                                    as_str(): "t6",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1981,
                                                                                end: 1982,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 1982,
                                                                                end: 1987,
                                                                                as_str(): "three",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 1988,
                                                                            end: 1990,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 1991,
                                                                                    end: 1994,
                                                                                    as_str(): "110",
                                                                                },
                                                                                parsed: 110,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1973,
                                                            end: 1995,
                                                            as_str(): "(s.t0.t6.three == 110)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 1995,
                                                            end: 1996,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 2001,
                                                                        end: 2007,
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
                                                                    lhs: TupleFieldProjection {
                                                                        target: Parens(
                                                                            Parens {
                                                                                inner: FieldProjection {
                                                                                    target: FieldProjection {
                                                                                        target: FieldProjection {
                                                                                            target: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2009,
                                                                                                                end: 2010,
                                                                                                                as_str(): "s",
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
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2010,
                                                                                                    end: 2011,
                                                                                                    as_str(): ".",
                                                                                                },
                                                                                            },
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2011,
                                                                                                    end: 2013,
                                                                                                    as_str(): "t0",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        dot_token: DotToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 2013,
                                                                                                end: 2014,
                                                                                                as_str(): ".",
                                                                                            },
                                                                                        },
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 2014,
                                                                                                end: 2016,
                                                                                                as_str(): "t6",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 2016,
                                                                                            end: 2017,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 2017,
                                                                                            end: 2021,
                                                                                            as_str(): "four",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 2008,
                                                                                    end: 2022,
                                                                                    as_str(): "(s.t0.t6.four)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 2022,
                                                                                end: 2023,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 2023,
                                                                            end: 2024,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 2025,
                                                                            end: 2027,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 2028,
                                                                                    end: 2031,
                                                                                    as_str(): "111",
                                                                                },
                                                                                parsed: 111,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 2007,
                                                            end: 2032,
                                                            as_str(): "((s.t0.t6.four).0 == 111)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 2032,
                                                            end: 2033,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 2038,
                                                                        end: 2044,
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
                                                                    lhs: TupleFieldProjection {
                                                                        target: Parens(
                                                                            Parens {
                                                                                inner: FieldProjection {
                                                                                    target: FieldProjection {
                                                                                        target: FieldProjection {
                                                                                            target: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 2046,
                                                                                                                end: 2047,
                                                                                                                as_str(): "s",
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
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2047,
                                                                                                    end: 2048,
                                                                                                    as_str(): ".",
                                                                                                },
                                                                                            },
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2048,
                                                                                                    end: 2050,
                                                                                                    as_str(): "t0",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        dot_token: DotToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 2050,
                                                                                                end: 2051,
                                                                                                as_str(): ".",
                                                                                            },
                                                                                        },
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                                ),
                                                                                                start: 2051,
                                                                                                end: 2053,
                                                                                                as_str(): "t6",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 2053,
                                                                                            end: 2054,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 2054,
                                                                                            end: 2058,
                                                                                            as_str(): "four",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 2045,
                                                                                    end: 2059,
                                                                                    as_str(): "(s.t0.t6.four)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 2059,
                                                                                end: 2060,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 2060,
                                                                            end: 2061,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 2062,
                                                                            end: 2064,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 2065,
                                                                                    end: 2068,
                                                                                    as_str(): "112",
                                                                                },
                                                                                parsed: 112,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 2044,
                                                            end: 2069,
                                                            as_str(): "((s.t0.t6.four).1 == 112)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 2069,
                                                            end: 2070,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c64be10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                        ),
                                                                        start: 2075,
                                                                        end: 2081,
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
                                                                                            src (ptr): 0x00007fe07c64be10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                            ),
                                                                                            start: 2082,
                                                                                            end: 2083,
                                                                                            as_str(): "s",
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
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 2083,
                                                                                end: 2084,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c64be10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                ),
                                                                                start: 2084,
                                                                                end: 2086,
                                                                                as_str(): "t3",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c64be10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                            ),
                                                                            start: 2087,
                                                                            end: 2089,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c64be10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                                                    ),
                                                                                    start: 2090,
                                                                                    end: 2093,
                                                                                    as_str(): "113",
                                                                                },
                                                                                parsed: 113,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 2081,
                                                            end: 2094,
                                                            as_str(): "(s.t3 == 113)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 2094,
                                                            end: 2095,
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
                                                            src (ptr): 0x00007fe07c64be10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                                            ),
                                                            start: 2101,
                                                            end: 2105,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe07c64be10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFpAlTb/nested_structs/src/main.sw",
                                        ),
                                        start: 402,
                                        end: 2107,
                                        as_str(): "{\n    let mut s = S {\n        t0: W {\n            t5: 5,\n            t6: T6 {\n                zero: 6,\n                one: 7, \n                two: I {\n                    zero: 8, \n                    one: 9\n                },\n                three: 10,\n                four: (11, 12)\n            }\n        },\n        t1: T1 {\n            zero: 0, \n            one: 1\n        },\n        t2: T {\n            t3: T3 {\n                zero: 2, \n                one: 3\n            },\n            t4: 4\n        },\n        t3: 13\n    };\n\n    assert(s.t1.zero == 0);\n    assert(s.t1.one == 1);\n    assert(s.t2.t3.zero == 2);\n    assert(s.t2.t3.one == 3);\n    assert(s.t2.t4 == 4);\n    assert(s.t0.t5 == 5);\n    assert(s.t0.t6.zero == 6);\n    assert(s.t0.t6.one == 7);\n    assert(s.t0.t6.two.zero == 8);\n    assert(s.t0.t6.two.one == 9);\n    assert(s.t0.t6.three == 10);\n    assert((s.t0.t6.four).0 == 11);\n    assert((s.t0.t6.four).1 == 12);\n    assert(s.t3 == 13);\n\n    s.t1.zero = 10;\n    s.t1.one = 11;\n    s.t2.t3.zero = 12;\n    s.t2.t3.one = 13;\n    s.t2.t4 = 14;\n    s.t0.t5 = 15;\n    s.t0.t6.zero = 16;\n    s.t0.t6.one = 17;\n    s.t0.t6.two.zero = 18;\n    s.t0.t6.two.one = 19;\n    s.t0.t6.three = 110;\n    s.t0.t6.four = (111, 112);\n    s.t3 = 113;\n\n    assert(s.t1.zero == 10);\n    assert(s.t1.one == 11);\n    assert(s.t2.t3.zero == 12);\n    assert(s.t2.t3.one == 13);\n    assert(s.t2.t4 == 14);\n    assert(s.t0.t5 == 15);\n    assert(s.t0.t6.zero == 16);\n    assert(s.t0.t6.one == 17);\n    assert(s.t0.t6.two.zero == 18);\n    assert(s.t0.t6.two.one == 19);\n    assert(s.t0.t6.three == 110);\n    assert((s.t0.t6.four).0 == 111);\n    assert((s.t0.t6.four).1 == 112);\n    assert(s.t3 == 113);\n\n    true\n}",
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
