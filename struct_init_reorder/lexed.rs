Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe03e568ce0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe03e568ce0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
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
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
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
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
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
                                                src (ptr): 0x00007fe03e568ce0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 25,
                                                as_str(): "logging",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe03e568ce0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                ),
                                                start: 25,
                                                end: 27,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe03e568ce0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                    ),
                                                    start: 27,
                                                    end: 30,
                                                    as_str(): "log",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 30,
                                        end: 31,
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
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 33,
                                            end: 36,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 43,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 44,
                                        end: 57,
                                        as_str(): "MyInnerStruct",
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 66,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 67,
                                                                            end: 70,
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 70,
                                                        end: 71,
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 76,
                                                                end: 77,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 78,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 79,
                                                                            end: 82,
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 82,
                                                        end: 83,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 58,
                                        end: 85,
                                        as_str(): "{\n    x: u64,\n    y: u64,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 87,
                                            end: 90,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 91,
                                        end: 97,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 106,
                                        as_str(): "MyStruct",
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 115,
                                                                end: 120,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 120,
                                                                end: 121,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 122,
                                                                            end: 135,
                                                                            as_str(): "MyInnerStruct",
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 135,
                                                        end: 136,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 107,
                                        end: 138,
                                        as_str(): "{\n      value: MyInnerStruct,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 140,
                                            end: 143,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 144,
                                        end: 148,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 149,
                                        end: 155,
                                        as_str(): "MyEnum",
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 162,
                                                                end: 164,
                                                                as_str(): "V1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 164,
                                                                end: 165,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 166,
                                                                            end: 168,
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 168,
                                                        end: 169,
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 174,
                                                                end: 176,
                                                                as_str(): "V2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 176,
                                                                end: 177,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 178,
                                                                            end: 181,
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 181,
                                                        end: 182,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 156,
                                        end: 184,
                                        as_str(): "{\n    V1: u8,\n    V2: u64,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 189,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 190,
                                        end: 196,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 197,
                                        end: 200,
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 207,
                                                                end: 209,
                                                                as_str(): "f1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 209,
                                                                end: 210,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 211,
                                                                            end: 217,
                                                                            as_str(): "MyEnum",
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 217,
                                                        end: 218,
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 223,
                                                                end: 225,
                                                                as_str(): "f2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 225,
                                                                end: 226,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 227,
                                                                            end: 235,
                                                                            as_str(): "MyStruct",
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
                                                        src (ptr): 0x00007fe03e568ce0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                        ),
                                                        start: 235,
                                                        end: 236,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 201,
                                        end: 238,
                                        as_str(): "{\n    f1: MyEnum,\n    f2: MyStruct,\n}",
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
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 240,
                                            end: 242,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 243,
                                            end: 247,
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
                                            src (ptr): 0x00007fe03e568ce0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                            ),
                                            start: 247,
                                            end: 249,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 256,
                                                            end: 259,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 260,
                                                                end: 262,
                                                                as_str(): "f1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 263,
                                                                    end: 264,
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
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 265,
                                                                                end: 271,
                                                                                as_str(): "MyEnum",
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
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 272,
                                                            end: 273,
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
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 274,
                                                                            end: 280,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 280,
                                                                                end: 282,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                    ),
                                                                                    start: 282,
                                                                                    end: 284,
                                                                                    as_str(): "V1",
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
                                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                    ),
                                                                                    start: 285,
                                                                                    end: 286,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U8,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 286,
                                                                                            end: 288,
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
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 284,
                                                                end: 289,
                                                                as_str(): "(0u8)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 289,
                                                            end: 290,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 295,
                                                            end: 298,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 299,
                                                                end: 301,
                                                                as_str(): "f2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                    ),
                                                                    start: 302,
                                                                    end: 303,
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
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 304,
                                                                                end: 312,
                                                                                as_str(): "MyStruct",
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
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 313,
                                                            end: 314,
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
                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                        ),
                                                                        start: 315,
                                                                        end: 323,
                                                                        as_str(): "MyStruct",
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
                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                ),
                                                                                start: 326,
                                                                                end: 331,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                        ),
                                                                                        start: 331,
                                                                                        end: 332,
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
                                                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                    ),
                                                                                                    start: 333,
                                                                                                    end: 346,
                                                                                                    as_str(): "MyInnerStruct",
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
                                                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                                ),
                                                                                                                start: 349,
                                                                                                                end: 350,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 350,
                                                                                                                        end: 351,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 352,
                                                                                                                                end: 353,
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
                                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                            ),
                                                                                                            start: 353,
                                                                                                            end: 354,
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
                                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                            ),
                                                                                                            start: 355,
                                                                                                            end: 356,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    expr_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 356,
                                                                                                                    end: 357,
                                                                                                                    as_str(): ":",
                                                                                                                },
                                                                                                            },
                                                                                                            Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 358,
                                                                                                                            end: 359,
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
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 347,
                                                                                            end: 361,
                                                                                            as_str(): "{ x: 0, y: 0 }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe03e568ce0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                ),
                                                                start: 324,
                                                                end: 363,
                                                                as_str(): "{ value: MyInnerStruct { x: 0, y: 0 } }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 363,
                                                            end: 364,
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
                                                                        src (ptr): 0x00007fe03e568ce0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                        ),
                                                                        start: 452,
                                                                        end: 455,
                                                                        as_str(): "log",
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
                                                                Struct {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe03e568ce0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                    ),
                                                                                    start: 456,
                                                                                    end: 459,
                                                                                    as_str(): "Foo",
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
                                                                                                src (ptr): 0x00007fe03e568ce0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                                ),
                                                                                                start: 470,
                                                                                                end: 472,
                                                                                                as_str(): "f2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: None,
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 472,
                                                                                            end: 473,
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
                                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                                            ),
                                                                                            start: 482,
                                                                                            end: 484,
                                                                                            as_str(): "f1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    expr_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe03e568ce0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                                            ),
                                                                            start: 460,
                                                                            end: 490,
                                                                            as_str(): "{\n        f2,\n        f1\n    }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 455,
                                                            end: 491,
                                                            as_str(): "(Foo {\n        f2,\n        f1\n    })",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe03e568ce0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                                            ),
                                                            start: 491,
                                                            end: 492,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe03e568ce0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIQBKdB/struct_init_reorder/src/main.sw",
                                        ),
                                        start: 250,
                                        end: 494,
                                        as_str(): "{\n    let f1 : MyEnum = MyEnum::V1(0u8);\n    let f2 : MyStruct = MyStruct { value: MyInnerStruct { x: 0, y: 0 } };\n    // f1 and f2 are instantiated in the wrong order below. that shouldn't matter.\n    log(Foo {\n        f2,\n        f1\n    });\n}",
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
