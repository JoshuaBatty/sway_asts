Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb101998d50,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb101998d50,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
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
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 170,
                                        end: 174,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 175,
                                        end: 181,
                                        as_str(): "Result",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 182,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 182,
                                                                end: 183,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 183,
                                                                end: 184,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 186,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 186,
                                                    end: 187,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 194,
                                                                end: 196,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 197,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 198,
                                                                            end: 199,
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 199,
                                                        end: 200,
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 205,
                                                                end: 208,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 208,
                                                                end: 209,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 210,
                                                                            end: 211,
                                                                            as_str(): "E",
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 211,
                                                        end: 212,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 214,
                                        as_str(): "{\n    Ok: T,\n    Err: E,\n}",
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
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 216,
                                        end: 222,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 223,
                                        end: 230,
                                        as_str(): "Product",
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 237,
                                                                end: 244,
                                                                as_str(): "details",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 244,
                                                                end: 245,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 246,
                                                                            end: 257,
                                                                            as_str(): "ItemDetails",
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 257,
                                                        end: 258,
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 263,
                                                                end: 279,
                                                                as_str(): "inventory_number",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 279,
                                                                end: 280,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 281,
                                                                            end: 284,
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 284,
                                                        end: 285,
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 290,
                                                                end: 301,
                                                                as_str(): "number_sold",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 306,
                                                        end: 307,
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 312,
                                                                end: 328,
                                                                as_str(): "number_available",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 328,
                                                                end: 329,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 330,
                                                                            end: 333,
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 333,
                                                        end: 334,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 231,
                                        end: 336,
                                        as_str(): "{\n    details: ItemDetails,\n    inventory_number: u64,\n    number_sold: u64,\n    number_available: u64,\n}",
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
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 338,
                                        end: 344,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 345,
                                        end: 356,
                                        as_str(): "ItemDetails",
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 363,
                                                                end: 367,
                                                                as_str(): "name",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 367,
                                                                end: 368,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 369,
                                                                    end: 372,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 373,
                                                                                end: 374,
                                                                                as_str(): "4",
                                                                            },
                                                                            parsed: 4,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 372,
                                                                    end: 375,
                                                                    as_str(): "[4]",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 375,
                                                        end: 376,
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 381,
                                                                end: 386,
                                                                as_str(): "price",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 386,
                                                                end: 387,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 388,
                                                                            end: 391,
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
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 391,
                                                        end: 392,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 357,
                                        end: 394,
                                        as_str(): "{\n    name: str[4],\n    price: u64,\n}",
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
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 396,
                                        end: 400,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 401,
                                        end: 410,
                                        as_str(): "SaleError",
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 417,
                                                                end: 435,
                                                                as_str(): "NotEnoughInventory",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 435,
                                                                end: 436,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 437,
                                                                    end: 440,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 441,
                                                                                end: 442,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 440,
                                                                    end: 443,
                                                                    as_str(): "[3]",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb101998d50,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                        ),
                                                        start: 443,
                                                        end: 444,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 411,
                                        end: 446,
                                        as_str(): "{\n    NotEnoughInventory: str[3],\n}",
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
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 448,
                                            end: 450,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 451,
                                            end: 455,
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
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 455,
                                            end: 457,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 458,
                                                    end: 460,
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 461,
                                                                end: 464,
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
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 471,
                                                            end: 474,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 475,
                                                                end: 476,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 477,
                                                            end: 478,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 479,
                                                                            end: 491,
                                                                            as_str(): "sell_product",
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
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 492,
                                                                                        end: 499,
                                                                                        as_str(): "Product",
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
                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 510,
                                                                                                    end: 517,
                                                                                                    as_str(): "details",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 517,
                                                                                                            end: 518,
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
                                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 519,
                                                                                                                        end: 530,
                                                                                                                        as_str(): "ItemDetails",
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
                                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 545,
                                                                                                                                    end: 549,
                                                                                                                                    as_str(): "name",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            expr_opt: Some(
                                                                                                                                (
                                                                                                                                    ColonToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 549,
                                                                                                                                            end: 550,
                                                                                                                                            as_str(): ":",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Literal(
                                                                                                                                        String(
                                                                                                                                            LitString {
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 551,
                                                                                                                                                    end: 557,
                                                                                                                                                    as_str(): "\"shoe\"",
                                                                                                                                                },
                                                                                                                                                parsed: "shoe",
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        CommaToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 557,
                                                                                                                                end: 558,
                                                                                                                                as_str(): ",",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    (
                                                                                                                        ExprStructField {
                                                                                                                            field_name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 559,
                                                                                                                                    end: 564,
                                                                                                                                    as_str(): "price",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            expr_opt: Some(
                                                                                                                                (
                                                                                                                                    ColonToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 564,
                                                                                                                                            end: 565,
                                                                                                                                            as_str(): ":",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Literal(
                                                                                                                                        Int(
                                                                                                                                            LitInt {
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 566,
                                                                                                                                                    end: 569,
                                                                                                                                                    as_str(): "100",
                                                                                                                                                },
                                                                                                                                                parsed: 100,
                                                                                                                                                ty_opt: None,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        CommaToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 569,
                                                                                                                                end: 570,
                                                                                                                                as_str(): ",",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ],
                                                                                                                final_value_opt: None,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 531,
                                                                                                                end: 581,
                                                                                                                as_str(): "{\n            name: \"shoe\", price: 100, \n        }",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                ),
                                                                                                start: 581,
                                                                                                end: 582,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        ExprStructField {
                                                                                            field_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 591,
                                                                                                    end: 607,
                                                                                                    as_str(): "inventory_number",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 607,
                                                                                                            end: 608,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 609,
                                                                                                                    end: 610,
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
                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                ),
                                                                                                start: 610,
                                                                                                end: 611,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    (
                                                                                        ExprStructField {
                                                                                            field_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 612,
                                                                                                    end: 623,
                                                                                                    as_str(): "number_sold",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 623,
                                                                                                            end: 624,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 625,
                                                                                                                    end: 627,
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
                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                ),
                                                                                                start: 627,
                                                                                                end: 628,
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
                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                ),
                                                                                                start: 629,
                                                                                                end: 645,
                                                                                                as_str(): "number_available",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 645,
                                                                                                        end: 646,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 647,
                                                                                                                end: 648,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 500,
                                                                                end: 654,
                                                                                as_str(): "{\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    }",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 491,
                                                                end: 655,
                                                                as_str(): "(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    })",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 655,
                                                            end: 656,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 686,
                                                            end: 688,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Let {
                                                        let_token: LetToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 689,
                                                                end: 692,
                                                                as_str(): "let",
                                                            },
                                                        },
                                                        lhs: Constructor {
                                                            path: PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 699,
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
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 699,
                                                                                end: 701,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 701,
                                                                                    end: 703,
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
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 704,
                                                                                    end: 705,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 703,
                                                                    end: 706,
                                                                    as_str(): "(y)",
                                                                },
                                                            },
                                                        },
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 707,
                                                                end: 708,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        rhs: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 709,
                                                                            end: 710,
                                                                            as_str(): "x",
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
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Add {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 721,
                                                                                            end: 722,
                                                                                            as_str(): "y",
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
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 722,
                                                                                end: 723,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 723,
                                                                                end: 734,
                                                                                as_str(): "number_sold",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    add_token: AddToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 735,
                                                                            end: 736,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 737,
                                                                                    end: 738,
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
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 711,
                                                            end: 744,
                                                            as_str(): "{\n        y.number_sold + 4\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 745,
                                                                    end: 749,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 760,
                                                                                            end: 761,
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
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 750,
                                                                        end: 767,
                                                                        as_str(): "{\n        1\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 465,
                                        end: 769,
                                        as_str(): "{\n    let x = sell_product(Product {\n        details: ItemDetails {\n            name: \"shoe\", price: 100, \n        },\n        inventory_number: 0, number_sold: 10, number_available: 5\n    });\n\n    // should return 15\n    if let Result::Ok(y) = x {\n        y.number_sold + 4\n    } else {\n        1\n    }\n}",
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
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 771,
                                            end: 773,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 774,
                                            end: 786,
                                            as_str(): "sell_product",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 787,
                                                                    end: 794,
                                                                    as_str(): "product",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 794,
                                                                end: 795,
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 796,
                                                                            end: 803,
                                                                            as_str(): "Product",
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
                                        span: Span {
                                            src (ptr): 0x00007fb101998d50,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                            ),
                                            start: 786,
                                            end: 804,
                                            as_str(): "(product: Product)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb101998d50,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                    ),
                                                    start: 805,
                                                    end: 807,
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
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 808,
                                                                end: 814,
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
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 814,
                                                                                end: 815,
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
                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 815,
                                                                                                        end: 822,
                                                                                                        as_str(): "Product",
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
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 822,
                                                                                            end: 823,
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
                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 824,
                                                                                                    end: 833,
                                                                                                    as_str(): "SaleError",
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
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 833,
                                                                                end: 834,
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
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 841,
                                                            end: 844,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 845,
                                                                    end: 848,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 849,
                                                                end: 856,
                                                                as_str(): "product",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 857,
                                                            end: 858,
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
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 859,
                                                                        end: 866,
                                                                        as_str(): "product",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 866,
                                                            end: 867,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 872,
                                                                end: 874,
                                                                as_str(): "if",
                                                            },
                                                        },
                                                        condition: Expr(
                                                            LessThan {
                                                                lhs: FieldProjection {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 875,
                                                                                        end: 882,
                                                                                        as_str(): "product",
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
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 882,
                                                                            end: 883,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb101998d50,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                            ),
                                                                            start: 883,
                                                                            end: 899,
                                                                            as_str(): "number_available",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                less_than_token: LessThanToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 900,
                                                                        end: 901,
                                                                        as_str(): "<",
                                                                    },
                                                                },
                                                                rhs: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 902,
                                                                                end: 903,
                                                                                as_str(): "1",
                                                                            },
                                                                            parsed: 1,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                        then_block: Braces {
                                                            inner: CodeBlockContents {
                                                                statements: [
                                                                    Expr {
                                                                        expr: Return {
                                                                            return_token: ReturnToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 914,
                                                                                    end: 920,
                                                                                    as_str(): "return",
                                                                                },
                                                                            },
                                                                            expr_opt: Some(
                                                                                FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 921,
                                                                                                        end: 927,
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
                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 927,
                                                                                                            end: 929,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 929,
                                                                                                                end: 932,
                                                                                                                as_str(): "Err",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: Some(
                                                                                                            (
                                                                                                                DoubleColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 932,
                                                                                                                        end: 934,
                                                                                                                        as_str(): "::",
                                                                                                                    },
                                                                                                                },
                                                                                                                GenericArgs {
                                                                                                                    parameters: AngleBrackets {
                                                                                                                        open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 934,
                                                                                                                                end: 935,
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
                                                                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 935,
                                                                                                                                                        end: 942,
                                                                                                                                                        as_str(): "Product",
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
                                                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 942,
                                                                                                                                            end: 943,
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
                                                                                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 944,
                                                                                                                                                    end: 953,
                                                                                                                                                    as_str(): "SaleError",
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
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 953,
                                                                                                                                end: 954,
                                                                                                                                as_str(): ">",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
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
                                                                                                FuncApp {
                                                                                                    func: Path(
                                                                                                        PathExpr {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 955,
                                                                                                                        end: 964,
                                                                                                                        as_str(): "SaleError",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                generics_opt: None,
                                                                                                            },
                                                                                                            suffix: [
                                                                                                                (
                                                                                                                    DoubleColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 964,
                                                                                                                            end: 966,
                                                                                                                            as_str(): "::",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 966,
                                                                                                                                end: 984,
                                                                                                                                as_str(): "NotEnoughInventory",
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
                                                                                                                    String(
                                                                                                                        LitString {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb101998d50,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 985,
                                                                                                                                end: 990,
                                                                                                                                as_str(): "\"noo\"",
                                                                                                                            },
                                                                                                                            parsed: "noo",
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 984,
                                                                                                            end: 991,
                                                                                                            as_str(): "(\"noo\")",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 954,
                                                                                            end: 992,
                                                                                            as_str(): "(SaleError::NotEnoughInventory(\"noo\"))",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 992,
                                                                                    end: 993,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 904,
                                                                end: 999,
                                                                as_str(): "{\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    }",
                                                            },
                                                        },
                                                        else_opt: None,
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 999,
                                                            end: 1000,
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
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1005,
                                                                    end: 1012,
                                                                    as_str(): "product",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1012,
                                                                end: 1013,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1013,
                                                                end: 1024,
                                                                as_str(): "number_sold",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1025,
                                                            end: 1026,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Add {
                                                        lhs: FieldProjection {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 1027,
                                                                                end: 1034,
                                                                                as_str(): "product",
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
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1034,
                                                                    end: 1035,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1035,
                                                                    end: 1046,
                                                                    as_str(): "number_sold",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        add_token: AddToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1047,
                                                                end: 1048,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 1049,
                                                                        end: 1050,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1050,
                                                            end: 1051,
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
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1056,
                                                                    end: 1063,
                                                                    as_str(): "product",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1063,
                                                                end: 1064,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1064,
                                                                end: 1080,
                                                                as_str(): "number_available",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1081,
                                                            end: 1082,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Sub {
                                                        lhs: FieldProjection {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 1083,
                                                                                end: 1090,
                                                                                as_str(): "product",
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
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1090,
                                                                    end: 1091,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1091,
                                                                    end: 1107,
                                                                    as_str(): "number_available",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        sub_token: SubToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb101998d50,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                ),
                                                                start: 1108,
                                                                end: 1109,
                                                                as_str(): "-",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb101998d50,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                        ),
                                                                        start: 1110,
                                                                        end: 1111,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1111,
                                                            end: 1112,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1117,
                                                            end: 1123,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        FuncApp {
                                                            func: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb101998d50,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                ),
                                                                                start: 1124,
                                                                                end: 1130,
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
                                                                                    src (ptr): 0x00007fb101998d50,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                    ),
                                                                                    start: 1130,
                                                                                    end: 1132,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb101998d50,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                        ),
                                                                                        start: 1132,
                                                                                        end: 1134,
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
                                                            ),
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
                                                                                            src (ptr): 0x00007fb101998d50,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1135,
                                                                                            end: 1142,
                                                                                            as_str(): "product",
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
                                                                    src (ptr): 0x00007fb101998d50,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                                    ),
                                                                    start: 1134,
                                                                    end: 1143,
                                                                    as_str(): "(product)",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb101998d50,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                                            ),
                                                            start: 1143,
                                                            end: 1144,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb101998d50,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRAS6jVS/enum_if_let_large_type/src/main.sw",
                                        ),
                                        start: 835,
                                        end: 1146,
                                        as_str(): "{\n    let mut product = product;\n    if product.number_available < 1 {\n        return Result::Err::<Product, SaleError>(SaleError::NotEnoughInventory(\"noo\"));\n    };\n    product.number_sold = product.number_sold + 1;\n    product.number_available = product.number_available - 1;\n    return Result::Ok(product);\n}",
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
