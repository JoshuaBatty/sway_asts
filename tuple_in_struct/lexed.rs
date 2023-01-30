Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a20324af0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a20324af0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
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
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 15,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
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
                                                src (ptr): 0x00007f8a20324af0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                ),
                                                start: 17,
                                                end: 23,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007f8a20324af0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
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
                                                    src (ptr): 0x00007f8a20324af0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 34,
                                        end: 40,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 42,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 49,
                                                                end: 51,
                                                                as_str(): "t5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 52,
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
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 53,
                                                                            end: 56,
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
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 56,
                                                        end: 57,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 62,
                                                                end: 64,
                                                                as_str(): "t6",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 67,
                                                                                        end: 69,
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
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 69,
                                                                            end: 70,
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
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 71,
                                                                                                    end: 73,
                                                                                                    as_str(): "u8",
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 73,
                                                                                        end: 74,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                Tuple(
                                                                                    Parens {
                                                                                        inner: Cons {
                                                                                            head: Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 76,
                                                                                                                end: 79,
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
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 79,
                                                                                                    end: 80,
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
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
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
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 75,
                                                                                            end: 84,
                                                                                            as_str(): "(u64, u8)",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 84,
                                                                                        end: 85,
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
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 86,
                                                                                                end: 89,
                                                                                                as_str(): "u16",
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
                                                                    src (ptr): 0x00007f8a20324af0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 90,
                                                                    as_str(): "(u8, u8, (u64, u8), u16)",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 90,
                                                        end: 91,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 93,
                                        as_str(): "{\n    t5: u64,\n    t6: (u8, u8, (u64, u8), u16),\n}",
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 101,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 102,
                                        end: 103,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 112,
                                                                as_str(): "t3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 112,
                                                                end: 113,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 115,
                                                                                        end: 117,
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
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 117,
                                                                            end: 118,
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
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 119,
                                                                                                end: 121,
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
                                                                    src (ptr): 0x00007f8a20324af0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 114,
                                                                    end: 122,
                                                                    as_str(): "(u8, u8)",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 122,
                                                        end: 123,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 130,
                                                            as_str(): "t4",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 131,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 132,
                                                                        end: 135,
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 104,
                                        end: 137,
                                        as_str(): "{\n    t3: (u8, u8),\n    t4: u16\n}",
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 139,
                                        end: 145,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 146,
                                        end: 147,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 154,
                                                                end: 156,
                                                                as_str(): "t0",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 156,
                                                                end: 157,
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
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 158,
                                                                            end: 159,
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
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 159,
                                                        end: 160,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 165,
                                                                end: 167,
                                                                as_str(): "t1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 167,
                                                                end: 168,
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
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 170,
                                                                                        end: 173,
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
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 173,
                                                                            end: 174,
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
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 175,
                                                                                                end: 178,
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
                                                                    src (ptr): 0x00007f8a20324af0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                    ),
                                                                    start: 169,
                                                                    end: 179,
                                                                    as_str(): "(u64, u64)",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 179,
                                                        end: 180,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 185,
                                                                end: 187,
                                                                as_str(): "t2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 187,
                                                                end: 188,
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
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 189,
                                                                            end: 190,
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
                                                        src (ptr): 0x00007f8a20324af0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                        ),
                                                        start: 190,
                                                        end: 191,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 148,
                                        end: 193,
                                        as_str(): "{\n    t0: W,\n    t1: (u64, u64),\n    t2: T,\n}",
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 195,
                                        end: 201,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 202,
                                        end: 203,
                                        as_str(): "U",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Annotated {
                                                attribute_list: [],
                                                value: TypeField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 210,
                                                            end: 211,
                                                            as_str(): "u",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 211,
                                                            end: 212,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 213,
                                                                        end: 216,
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
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 204,
                                        end: 218,
                                        as_str(): "{\n    u: u64\n}",
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
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 220,
                                            end: 222,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 223,
                                            end: 227,
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
                                            src (ptr): 0x00007f8a20324af0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                            ),
                                            start: 227,
                                            end: 229,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a20324af0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                    ),
                                                    start: 230,
                                                    end: 232,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 233,
                                                                end: 237,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 244,
                                                            end: 247,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 248,
                                                                end: 249,
                                                                as_str(): "s",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 250,
                                                            end: 251,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 252,
                                                                        end: 253,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 264,
                                                                                    end: 266,
                                                                                    as_str(): "t0",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 266,
                                                                                            end: 267,
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
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 268,
                                                                                                        end: 269,
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
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 284,
                                                                                                                    end: 286,
                                                                                                                    as_str(): "t5",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 286,
                                                                                                                            end: 287,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 288,
                                                                                                                                    end: 289,
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
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 289,
                                                                                                                end: 290,
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
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 303,
                                                                                                                end: 305,
                                                                                                                as_str(): "t6",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 305,
                                                                                                                        end: 306,
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
                                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 308,
                                                                                                                                            end: 309,
                                                                                                                                            as_str(): "6",
                                                                                                                                        },
                                                                                                                                        parsed: 6,
                                                                                                                                        ty_opt: None,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            comma_token: CommaToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 309,
                                                                                                                                    end: 310,
                                                                                                                                    as_str(): ",",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            tail: Punctuated {
                                                                                                                                value_separator_pairs: [
                                                                                                                                    (
                                                                                                                                        Literal(
                                                                                                                                            Int(
                                                                                                                                                LitInt {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 311,
                                                                                                                                                        end: 312,
                                                                                                                                                        as_str(): "7",
                                                                                                                                                    },
                                                                                                                                                    parsed: 7,
                                                                                                                                                    ty_opt: None,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        CommaToken {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 312,
                                                                                                                                                end: 313,
                                                                                                                                                as_str(): ",",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    (
                                                                                                                                        Tuple(
                                                                                                                                            Parens {
                                                                                                                                                inner: Cons {
                                                                                                                                                    head: Literal(
                                                                                                                                                        Int(
                                                                                                                                                            LitInt {
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 315,
                                                                                                                                                                    end: 316,
                                                                                                                                                                    as_str(): "8",
                                                                                                                                                                },
                                                                                                                                                                parsed: 8,
                                                                                                                                                                ty_opt: None,
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                    ),
                                                                                                                                                    comma_token: CommaToken {
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 316,
                                                                                                                                                            end: 317,
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
                                                                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 318,
                                                                                                                                                                            end: 319,
                                                                                                                                                                            as_str(): "9",
                                                                                                                                                                        },
                                                                                                                                                                        parsed: 9,
                                                                                                                                                                        ty_opt: None,
                                                                                                                                                                    },
                                                                                                                                                                ),
                                                                                                                                                            ),
                                                                                                                                                        ),
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 314,
                                                                                                                                                    end: 320,
                                                                                                                                                    as_str(): "(8, 9)",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        CommaToken {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 320,
                                                                                                                                                end: 321,
                                                                                                                                                as_str(): ",",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ],
                                                                                                                                final_value_opt: Some(
                                                                                                                                    Literal(
                                                                                                                                        Int(
                                                                                                                                            LitInt {
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 322,
                                                                                                                                                    end: 324,
                                                                                                                                                    as_str(): "10",
                                                                                                                                                },
                                                                                                                                                parsed: 10,
                                                                                                                                                ty_opt: None,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            },
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 307,
                                                                                                                            end: 325,
                                                                                                                            as_str(): "(6, 7, (8, 9), 10)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 270,
                                                                                                end: 335,
                                                                                                as_str(): "{\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 335,
                                                                                end: 336,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 345,
                                                                                    end: 347,
                                                                                    as_str(): "t1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 347,
                                                                                            end: 348,
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
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 350,
                                                                                                                end: 351,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                            parsed: 0,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 351,
                                                                                                        end: 352,
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
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 353,
                                                                                                                        end: 354,
                                                                                                                        as_str(): "1",
                                                                                                                    },
                                                                                                                    parsed: 1,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 349,
                                                                                                end: 355,
                                                                                                as_str(): "(0, 1)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 355,
                                                                                end: 356,
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
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 365,
                                                                                end: 367,
                                                                                as_str(): "t2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 367,
                                                                                        end: 368,
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
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 369,
                                                                                                    end: 370,
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
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 385,
                                                                                                                end: 387,
                                                                                                                as_str(): "t3",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 387,
                                                                                                                        end: 388,
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
                                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 390,
                                                                                                                                            end: 391,
                                                                                                                                            as_str(): "2",
                                                                                                                                        },
                                                                                                                                        parsed: 2,
                                                                                                                                        ty_opt: None,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            comma_token: CommaToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 391,
                                                                                                                                    end: 392,
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
                                                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 393,
                                                                                                                                                    end: 394,
                                                                                                                                                    as_str(): "3",
                                                                                                                                                },
                                                                                                                                                parsed: 3,
                                                                                                                                                ty_opt: None,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            },
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 389,
                                                                                                                            end: 395,
                                                                                                                            as_str(): "(2, 3)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 395,
                                                                                                            end: 396,
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
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 409,
                                                                                                            end: 411,
                                                                                                            as_str(): "t4",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    expr_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 411,
                                                                                                                    end: 412,
                                                                                                                    as_str(): ":",
                                                                                                                },
                                                                                                            },
                                                                                                            Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 413,
                                                                                                                            end: 414,
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
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 371,
                                                                                            end: 424,
                                                                                            as_str(): "{\n            t3: (2, 3),\n            t4: 4\n        }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 254,
                                                                end: 430,
                                                                as_str(): "{\n        t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        },\n        t1: (0, 1),\n        t2: T {\n            t3: (2, 3),\n            t4: 4\n        }\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 430,
                                                            end: 431,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 441,
                                                                        end: 447,
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
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 449,
                                                                                                        end: 450,
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
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 450,
                                                                                            end: 451,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 451,
                                                                                            end: 453,
                                                                                            as_str(): "t1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 448,
                                                                                    end: 454,
                                                                                    as_str(): "(s.t1)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 454,
                                                                                end: 455,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 455,
                                                                            end: 456,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 457,
                                                                            end: 459,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 460,
                                                                                    end: 461,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 447,
                                                            end: 462,
                                                            as_str(): "((s.t1).0 == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 462,
                                                            end: 463,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 468,
                                                                        end: 474,
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
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 476,
                                                                                                        end: 477,
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
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 477,
                                                                                            end: 478,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 478,
                                                                                            end: 480,
                                                                                            as_str(): "t1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 475,
                                                                                    end: 481,
                                                                                    as_str(): "(s.t1)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 481,
                                                                                end: 482,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 482,
                                                                            end: 483,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 486,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 487,
                                                                                    end: 488,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 474,
                                                            end: 489,
                                                            as_str(): "((s.t1).1 == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 489,
                                                            end: 490,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 495,
                                                                        end: 501,
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
                                                                                        target: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 503,
                                                                                                            end: 504,
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
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 504,
                                                                                                end: 505,
                                                                                                as_str(): ".",
                                                                                            },
                                                                                        },
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 505,
                                                                                                end: 507,
                                                                                                as_str(): "t2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 507,
                                                                                            end: 508,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 508,
                                                                                            end: 510,
                                                                                            as_str(): "t3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 502,
                                                                                    end: 511,
                                                                                    as_str(): "(s.t2.t3)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 511,
                                                                                end: 512,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 512,
                                                                            end: 513,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 514,
                                                                            end: 516,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 517,
                                                                                    end: 518,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 501,
                                                            end: 519,
                                                            as_str(): "((s.t2.t3).0 == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 519,
                                                            end: 520,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 525,
                                                                        end: 531,
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
                                                                                        target: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 533,
                                                                                                            end: 534,
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
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 534,
                                                                                                end: 535,
                                                                                                as_str(): ".",
                                                                                            },
                                                                                        },
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 535,
                                                                                                end: 537,
                                                                                                as_str(): "t2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 537,
                                                                                            end: 538,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 538,
                                                                                            end: 540,
                                                                                            as_str(): "t3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 532,
                                                                                    end: 541,
                                                                                    as_str(): "(s.t2.t3)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 541,
                                                                                end: 542,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 542,
                                                                            end: 543,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 544,
                                                                            end: 546,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 547,
                                                                                    end: 548,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 531,
                                                            end: 549,
                                                            as_str(): "((s.t2.t3).1 == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 549,
                                                            end: 550,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 555,
                                                                        end: 561,
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
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 562,
                                                                                                end: 563,
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
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 563,
                                                                                    end: 564,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 564,
                                                                                    end: 566,
                                                                                    as_str(): "t2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 566,
                                                                                end: 567,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 567,
                                                                                end: 569,
                                                                                as_str(): "t4",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 570,
                                                                            end: 572,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 573,
                                                                                    end: 574,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 561,
                                                            end: 575,
                                                            as_str(): "(s.t2.t4 == 4)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 575,
                                                            end: 576,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 581,
                                                                        end: 587,
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
                                                                    lhs: Parens(
                                                                        Parens {
                                                                            inner: FieldProjection {
                                                                                target: Parens(
                                                                                    Parens {
                                                                                        inner: FieldProjection {
                                                                                            target: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 590,
                                                                                                                end: 591,
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
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 591,
                                                                                                    end: 592,
                                                                                                    as_str(): ".",
                                                                                                },
                                                                                            },
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 592,
                                                                                                    end: 594,
                                                                                                    as_str(): "t0",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 589,
                                                                                            end: 595,
                                                                                            as_str(): "(s.t0)",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                dot_token: DotToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 595,
                                                                                        end: 596,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 596,
                                                                                        end: 598,
                                                                                        as_str(): "t5",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 588,
                                                                                end: 599,
                                                                                as_str(): "((s.t0).t5)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 600,
                                                                            end: 602,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 603,
                                                                                    end: 604,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 587,
                                                            end: 605,
                                                            as_str(): "(((s.t0).t5) == 5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 605,
                                                            end: 606,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 611,
                                                                        end: 617,
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
                                                                                    target: Parens(
                                                                                        Parens {
                                                                                            inner: FieldProjection {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 620,
                                                                                                                    end: 621,
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
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 621,
                                                                                                        end: 622,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 622,
                                                                                                        end: 624,
                                                                                                        as_str(): "t0",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 619,
                                                                                                end: 625,
                                                                                                as_str(): "(s.t0)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 625,
                                                                                            end: 626,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 626,
                                                                                            end: 628,
                                                                                            as_str(): "t6",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 618,
                                                                                    end: 629,
                                                                                    as_str(): "((s.t0).t6)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 629,
                                                                                end: 630,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 630,
                                                                            end: 631,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 632,
                                                                            end: 634,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 635,
                                                                                    end: 636,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 617,
                                                            end: 637,
                                                            as_str(): "(((s.t0).t6).0 == 6)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 637,
                                                            end: 638,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 643,
                                                                        end: 649,
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
                                                                                    target: Parens(
                                                                                        Parens {
                                                                                            inner: FieldProjection {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 652,
                                                                                                                    end: 653,
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
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 653,
                                                                                                        end: 654,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 654,
                                                                                                        end: 656,
                                                                                                        as_str(): "t0",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 651,
                                                                                                end: 657,
                                                                                                as_str(): "(s.t0)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 657,
                                                                                            end: 658,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 658,
                                                                                            end: 660,
                                                                                            as_str(): "t6",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 650,
                                                                                    end: 661,
                                                                                    as_str(): "((s.t0).t6)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 661,
                                                                                end: 662,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 662,
                                                                            end: 663,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 664,
                                                                            end: 666,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 667,
                                                                                    end: 668,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 649,
                                                            end: 669,
                                                            as_str(): "(((s.t0).t6).1 == 7)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 669,
                                                            end: 670,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 675,
                                                                        end: 681,
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
                                                                                inner: TupleFieldProjection {
                                                                                    target: Parens(
                                                                                        Parens {
                                                                                            inner: FieldProjection {
                                                                                                target: Parens(
                                                                                                    Parens {
                                                                                                        inner: FieldProjection {
                                                                                                            target: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 685,
                                                                                                                                end: 686,
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
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 686,
                                                                                                                    end: 687,
                                                                                                                    as_str(): ".",
                                                                                                                },
                                                                                                            },
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 687,
                                                                                                                    end: 689,
                                                                                                                    as_str(): "t0",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 684,
                                                                                                            end: 690,
                                                                                                            as_str(): "(s.t0)",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                dot_token: DotToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 690,
                                                                                                        end: 691,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 691,
                                                                                                        end: 693,
                                                                                                        as_str(): "t6",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 683,
                                                                                                end: 694,
                                                                                                as_str(): "((s.t0).t6)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 694,
                                                                                            end: 695,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    field: 2,
                                                                                    field_span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 695,
                                                                                        end: 696,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 682,
                                                                                    end: 697,
                                                                                    as_str(): "(((s.t0).t6).2)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 697,
                                                                                end: 698,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 0,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 698,
                                                                            end: 699,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 700,
                                                                            end: 702,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 703,
                                                                                    end: 704,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 681,
                                                            end: 705,
                                                            as_str(): "((((s.t0).t6).2).0 == 8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 705,
                                                            end: 706,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 711,
                                                                        end: 717,
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
                                                                                inner: TupleFieldProjection {
                                                                                    target: Parens(
                                                                                        Parens {
                                                                                            inner: FieldProjection {
                                                                                                target: Parens(
                                                                                                    Parens {
                                                                                                        inner: FieldProjection {
                                                                                                            target: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 721,
                                                                                                                                end: 722,
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
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 722,
                                                                                                                    end: 723,
                                                                                                                    as_str(): ".",
                                                                                                                },
                                                                                                            },
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 723,
                                                                                                                    end: 725,
                                                                                                                    as_str(): "t0",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 720,
                                                                                                            end: 726,
                                                                                                            as_str(): "(s.t0)",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                dot_token: DotToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 726,
                                                                                                        end: 727,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 727,
                                                                                                        end: 729,
                                                                                                        as_str(): "t6",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 719,
                                                                                                end: 730,
                                                                                                as_str(): "((s.t0).t6)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 730,
                                                                                            end: 731,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    field: 2,
                                                                                    field_span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 731,
                                                                                        end: 732,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 718,
                                                                                    end: 733,
                                                                                    as_str(): "(((s.t0).t6).2)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 733,
                                                                                end: 734,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 1,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 734,
                                                                            end: 735,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 736,
                                                                            end: 738,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 739,
                                                                                    end: 740,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 717,
                                                            end: 741,
                                                            as_str(): "((((s.t0).t6).2).1 == 9)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 741,
                                                            end: 742,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 747,
                                                                        end: 753,
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
                                                                                    target: Parens(
                                                                                        Parens {
                                                                                            inner: FieldProjection {
                                                                                                target: Path(
                                                                                                    PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 756,
                                                                                                                    end: 757,
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
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 757,
                                                                                                        end: 758,
                                                                                                        as_str(): ".",
                                                                                                    },
                                                                                                },
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                        ),
                                                                                                        start: 758,
                                                                                                        end: 760,
                                                                                                        as_str(): "t0",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 755,
                                                                                                end: 761,
                                                                                                as_str(): "(s.t0)",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 761,
                                                                                            end: 762,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 762,
                                                                                            end: 764,
                                                                                            as_str(): "t6",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 754,
                                                                                    end: 765,
                                                                                    as_str(): "((s.t0).t6)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 765,
                                                                                end: 766,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 3,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 766,
                                                                            end: 767,
                                                                            as_str(): "3",
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 768,
                                                                            end: 770,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 771,
                                                                                    end: 773,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 753,
                                                            end: 774,
                                                            as_str(): "(((s.t0).t6).3 == 10)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 774,
                                                            end: 775,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 781,
                                                            end: 784,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 785,
                                                                end: 786,
                                                                as_str(): "u",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 787,
                                                            end: 788,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 789,
                                                                        end: 790,
                                                                        as_str(): "U",
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
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 801,
                                                                                end: 802,
                                                                                as_str(): "u",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a20324af0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 802,
                                                                                        end: 803,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 804,
                                                                                                end: 806,
                                                                                                as_str(): "22",
                                                                                            },
                                                                                            parsed: 22,
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
                                                                src (ptr): 0x00007f8a20324af0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                ),
                                                                start: 791,
                                                                end: 813,
                                                                as_str(): "{\n        u: 22 \n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 813,
                                                            end: 814,
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
                                                                        src (ptr): 0x00007f8a20324af0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                        ),
                                                                        start: 819,
                                                                        end: 825,
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
                                                                                            src (ptr): 0x00007f8a20324af0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 826,
                                                                                            end: 827,
                                                                                            as_str(): "u",
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
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 827,
                                                                                end: 828,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a20324af0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                ),
                                                                                start: 828,
                                                                                end: 829,
                                                                                as_str(): "u",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a20324af0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                            ),
                                                                            start: 830,
                                                                            end: 832,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a20324af0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 833,
                                                                                    end: 835,
                                                                                    as_str(): "22",
                                                                                },
                                                                                parsed: 22,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 825,
                                                            end: 836,
                                                            as_str(): "(u.u == 22)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 836,
                                                            end: 837,
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
                                                            src (ptr): 0x00007f8a20324af0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                                            ),
                                                            start: 843,
                                                            end: 847,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a20324af0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFvFs2I/tuple_in_struct/src/main.sw",
                                        ),
                                        start: 238,
                                        end: 849,
                                        as_str(): "{\n    let s = S {\n        t0: W {\n            t5: 5,\n            t6: (6, 7, (8, 9), 10)\n        },\n        t1: (0, 1),\n        t2: T {\n            t3: (2, 3),\n            t4: 4\n        }\n    };\n    \n    assert((s.t1).0 == 0);\n    assert((s.t1).1 == 1);\n    assert((s.t2.t3).0 == 2);\n    assert((s.t2.t3).1 == 3);\n    assert(s.t2.t4 == 4);\n    assert(((s.t0).t5) == 5);\n    assert(((s.t0).t6).0 == 6);\n    assert(((s.t0).t6).1 == 7);\n    assert((((s.t0).t6).2).0 == 8);\n    assert((((s.t0).t6).2).1 == 9);\n    assert(((s.t0).t6).3 == 10);\n\n    let u = U {\n        u: 22 \n    };\n    assert(u.u == 22);\n\n    true\n}",
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
