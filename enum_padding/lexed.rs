Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0ee12f040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0ee12f040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
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
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 12,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 13,
                                        end: 17,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 32,
                                        as_str(): "LowerLevelEnum",
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
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 39,
                                                                end: 44,
                                                                as_str(): "first",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 44,
                                                                end: 45,
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
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 46,
                                                                            end: 50,
                                                                            as_str(): "b256",
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
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 50,
                                                        end: 51,
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
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 56,
                                                                end: 62,
                                                                as_str(): "second",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
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
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 64,
                                                                            end: 67,
                                                                            as_str(): "u32",
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
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 67,
                                                        end: 68,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 70,
                                        as_str(): "{\n    first: b256,\n    second: u32,\n}",
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
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 72,
                                            end: 75,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 76,
                                        end: 82,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 83,
                                        end: 94,
                                        as_str(): "ThenAStruct",
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
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 101,
                                                                end: 106,
                                                                as_str(): "first",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 107,
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
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 108,
                                                                            end: 111,
                                                                            as_str(): "u32",
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
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 112,
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
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 117,
                                                                end: 123,
                                                                as_str(): "second",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 123,
                                                                end: 124,
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
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 125,
                                                                            end: 139,
                                                                            as_str(): "LowerLevelEnum",
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
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 139,
                                                        end: 140,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 142,
                                        as_str(): "{\n    first: u32,\n    second: LowerLevelEnum,\n}",
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
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 144,
                                            end: 147,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 148,
                                        end: 152,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 153,
                                        end: 165,
                                        as_str(): "TopLevelEnum",
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
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 172,
                                                                end: 177,
                                                                as_str(): "first",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 177,
                                                                end: 178,
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
                                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                        ),
                                                                                        start: 180,
                                                                                        end: 184,
                                                                                        as_str(): "b256",
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
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
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
                                                                                PathType {
                                                                                    root_opt: None,
                                                                                    prefix: PathTypeSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                ),
                                                                                                start: 190,
                                                                                                end: 194,
                                                                                                as_str(): "b256",
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
                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                    ),
                                                                    start: 179,
                                                                    end: 195,
                                                                    as_str(): "(b256,\n    b256)",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 195,
                                                        end: 196,
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
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 203,
                                                                as_str(): "second",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 203,
                                                                end: 204,
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
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 205,
                                                                            end: 216,
                                                                            as_str(): "ThenAStruct",
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
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 216,
                                                        end: 217,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 166,
                                        end: 219,
                                        as_str(): "{\n    first: (b256,\n    b256), second: ThenAStruct,\n}",
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
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 221,
                                            end: 223,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 224,
                                            end: 228,
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
                                            src (ptr): 0x00007fb0ee12f040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                            ),
                                            start: 228,
                                            end: 230,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0ee12f040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                    ),
                                                    start: 231,
                                                    end: 233,
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
                                                                src (ptr): 0x00007fb0ee12f040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                ),
                                                                start: 234,
                                                                end: 246,
                                                                as_str(): "TopLevelEnum",
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
                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                    ),
                                                                    start: 840,
                                                                    end: 852,
                                                                    as_str(): "TopLevelEnum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [
                                                            (
                                                                DoubleColonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 852,
                                                                        end: 854,
                                                                        as_str(): "::",
                                                                    },
                                                                },
                                                                PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                            ),
                                                                            start: 854,
                                                                            end: 860,
                                                                            as_str(): "second",
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
                                                            Struct {
                                                                path: PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                ),
                                                                                start: 861,
                                                                                end: 872,
                                                                                as_str(): "ThenAStruct",
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
                                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                            ),
                                                                                            start: 883,
                                                                                            end: 888,
                                                                                            as_str(): "first",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                    ),
                                                                                                    start: 888,
                                                                                                    end: 889,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0ee12f040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                            ),
                                                                                                            start: 890,
                                                                                                            end: 892,
                                                                                                            as_str(): "42",
                                                                                                        },
                                                                                                        parsed: 42,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                        ),
                                                                                        start: 892,
                                                                                        end: 893,
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
                                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                        ),
                                                                                        start: 894,
                                                                                        end: 900,
                                                                                        as_str(): "second",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                expr_opt: Some(
                                                                                    (
                                                                                        ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                ),
                                                                                                start: 900,
                                                                                                end: 901,
                                                                                                as_str(): ":",
                                                                                            },
                                                                                        },
                                                                                        FuncApp {
                                                                                            func: Path(
                                                                                                PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0ee12f040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                                ),
                                                                                                                start: 902,
                                                                                                                end: 916,
                                                                                                                as_str(): "LowerLevelEnum",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [
                                                                                                        (
                                                                                                            DoubleColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 916,
                                                                                                                    end: 918,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 918,
                                                                                                                        end: 924,
                                                                                                                        as_str(): "second",
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
                                                                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 925,
                                                                                                                        end: 927,
                                                                                                                        as_str(): "66",
                                                                                                                    },
                                                                                                                    parsed: 66,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0ee12f040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                                                    ),
                                                                                                    start: 924,
                                                                                                    end: 928,
                                                                                                    as_str(): "(66)",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0ee12f040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                                        ),
                                                                        start: 873,
                                                                        end: 934,
                                                                        as_str(): "{\n        first: 42, second: LowerLevelEnum::second(66)\n    }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ee12f040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                                        ),
                                                        start: 860,
                                                        end: 935,
                                                        as_str(): "(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ee12f040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXicyMJ/enum_padding/src/main.sw",
                                        ),
                                        start: 247,
                                        end: 937,
                                        as_str(): "{\n    // Expected output:\n    //\n    //  0000000000000001  # TopLevelEnum.tag\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  0000000000000000  #     TopLevelEnum.padding\n    //  000000000000002a  #     ThenAStruct.first(42)\n    //  0000000000000001  #     ThenAStruct.LowerLevelEnum.tag\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000000  #         ThenAStruct.LowerLevelEnum.padding\n    //  0000000000000042  #         ThenAStruct.LowerLevelEnum.second(66)\n\n    TopLevelEnum::second(ThenAStruct {\n        first: 42, second: LowerLevelEnum::second(66)\n    })\n}",
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
