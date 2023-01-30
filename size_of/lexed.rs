Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe04328bd30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe04328bd30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                                                src (ptr): 0x00007fe04328bd30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe04328bd30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 71,
                                        end: 77,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 78,
                                        end: 82,
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 87,
                                                                end: 90,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 90,
                                                                end: 91,
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 92,
                                                                            end: 95,
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
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 95,
                                                        end: 96,
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 99,
                                                                end: 102,
                                                                as_str(): "two",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 104,
                                                                            end: 107,
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
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 107,
                                                        end: 108,
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 111,
                                                                end: 116,
                                                                as_str(): "three",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 117,
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 118,
                                                                            end: 121,
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
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 121,
                                                        end: 122,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 83,
                                        end: 124,
                                        as_str(): "{\n  one: u64,\n  two: u64,\n  three: u64,\n}",
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
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 163,
                                        end: 169,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 170,
                                        end: 175,
                                        as_str(): "Point",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 180,
                                                                end: 181,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 183,
                                                                            end: 185,
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
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 185,
                                                        end: 186,
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 189,
                                                                end: 190,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 190,
                                                                end: 191,
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 192,
                                                                            end: 194,
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
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 194,
                                                        end: 195,
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 198,
                                                                end: 199,
                                                                as_str(): "z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 199,
                                                                end: 200,
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 201,
                                                                            end: 203,
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
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 203,
                                                        end: 204,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 176,
                                        end: 206,
                                        as_str(): "{\n  x: u8,\n  y: u8,\n  z: u8,\n}",
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
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 208,
                                            end: 210,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 211,
                                            end: 226,
                                            as_str(): "return_the_same",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 226,
                                                        end: 227,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 227,
                                                                end: 228,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe04328bd30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                        ),
                                                        start: 228,
                                                        end: 229,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
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
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 230,
                                                                    end: 234,
                                                                    as_str(): "elem",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 234,
                                                                end: 235,
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 236,
                                                                            end: 237,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 229,
                                            end: 238,
                                            as_str(): "(elem: T)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 239,
                                                    end: 241,
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 242,
                                                                end: 243,
                                                                as_str(): "T",
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
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 248,
                                                            end: 251,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 252,
                                                                end: 253,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04328bd30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                    ),
                                                                    start: 253,
                                                                    end: 254,
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
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 255,
                                                                                end: 256,
                                                                                as_str(): "T",
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
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 257,
                                                            end: 258,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 259,
                                                                        end: 263,
                                                                        as_str(): "elem",
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
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 263,
                                                            end: 264,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 267,
                                                                end: 268,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 244,
                                        end: 270,
                                        as_str(): "{\n  let x: T = elem;\n  x\n}",
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
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 272,
                                            end: 274,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 275,
                                            end: 279,
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
                                            src (ptr): 0x00007fe04328bd30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                            ),
                                            start: 279,
                                            end: 281,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe04328bd30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                    ),
                                                    start: 282,
                                                    end: 284,
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 285,
                                                                end: 288,
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
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
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
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 299,
                                                                end: 300,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 301,
                                                            end: 302,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 307,
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
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 318,
                                                                                    end: 321,
                                                                                    as_str(): "one",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 321,
                                                                                            end: 322,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 323,
                                                                                                    end: 324,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 324,
                                                                                end: 325,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 334,
                                                                                    end: 337,
                                                                                    as_str(): "two",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 337,
                                                                                            end: 338,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 339,
                                                                                                    end: 340,
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
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 340,
                                                                                end: 341,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 350,
                                                                                    end: 355,
                                                                                    as_str(): "three",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 355,
                                                                                            end: 356,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 357,
                                                                                                    end: 358,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 358,
                                                                                end: 359,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 308,
                                                                end: 365,
                                                                as_str(): "{\n        one: 1,\n        two: 2,\n        three: 3,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 365,
                                                            end: 366,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 371,
                                                            end: 374,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 375,
                                                                end: 376,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 377,
                                                            end: 378,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 379,
                                                                        end: 383,
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
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 397,
                                                                                    as_str(): "one",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 397,
                                                                                            end: 398,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 399,
                                                                                                    end: 404,
                                                                                                    as_str(): "10000",
                                                                                                },
                                                                                                parsed: 10000,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 404,
                                                                                end: 405,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 414,
                                                                                    end: 417,
                                                                                    as_str(): "two",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 417,
                                                                                            end: 418,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 419,
                                                                                                    end: 424,
                                                                                                    as_str(): "20000",
                                                                                                },
                                                                                                parsed: 20000,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 424,
                                                                                end: 425,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 434,
                                                                                    end: 439,
                                                                                    as_str(): "three",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 439,
                                                                                            end: 440,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 441,
                                                                                                    end: 446,
                                                                                                    as_str(): "30000",
                                                                                                },
                                                                                                parsed: 30000,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 446,
                                                                                end: 447,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 384,
                                                                end: 453,
                                                                as_str(): "{\n        one: 10000,\n        two: 20000,\n        three: 30000,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 453,
                                                            end: 454,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 459,
                                                            end: 462,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 463,
                                                                end: 464,
                                                                as_str(): "p",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 465,
                                                            end: 466,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 467,
                                                                        end: 472,
                                                                        as_str(): "Point",
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
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 481,
                                                                                    end: 482,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 482,
                                                                                            end: 483,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 484,
                                                                                                    end: 485,
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
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 485,
                                                                                end: 486,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 493,
                                                                                    end: 494,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 494,
                                                                                            end: 495,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 496,
                                                                                                    end: 497,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 497,
                                                                                end: 498,
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
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 505,
                                                                                end: 506,
                                                                                as_str(): "z",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                        ),
                                                                                        start: 506,
                                                                                        end: 507,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                ),
                                                                                                start: 508,
                                                                                                end: 509,
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
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 473,
                                                                end: 515,
                                                                as_str(): "{\n      x: 0,\n      y: 1,\n      z: 2\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 515,
                                                            end: 516,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 521,
                                                            end: 524,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 525,
                                                                end: 528,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 529,
                                                            end: 530,
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
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 531,
                                                                            end: 546,
                                                                            as_str(): "return_the_same",
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
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 547,
                                                                                    end: 548,
                                                                                    as_str(): "7",
                                                                                },
                                                                                parsed: 7,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 548,
                                                                                            end: 551,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04328bd30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                ),
                                                                start: 546,
                                                                end: 552,
                                                                as_str(): "(7u64)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 552,
                                                            end: 553,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 558,
                                                                        end: 564,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 565,
                                                                                            end: 578,
                                                                                            as_str(): "__size_of_val",
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
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                        ),
                                                                                                        start: 579,
                                                                                                        end: 580,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 578,
                                                                                end: 581,
                                                                                as_str(): "(x)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 582,
                                                                            end: 584,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 585,
                                                                                    end: 587,
                                                                                    as_str(): "24",
                                                                                },
                                                                                parsed: 24,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 564,
                                                            end: 588,
                                                            as_str(): "(__size_of_val(x) == 24)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 588,
                                                            end: 589,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 594,
                                                                        end: 600,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 601,
                                                                                            end: 614,
                                                                                            as_str(): "__size_of_val",
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
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                        ),
                                                                                                        start: 615,
                                                                                                        end: 616,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 614,
                                                                                end: 617,
                                                                                as_str(): "(y)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 618,
                                                                            end: 620,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 621,
                                                                                    end: 623,
                                                                                    as_str(): "24",
                                                                                },
                                                                                parsed: 24,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 600,
                                                            end: 624,
                                                            as_str(): "(__size_of_val(y) == 24)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 624,
                                                            end: 625,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 630,
                                                                        end: 636,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 637,
                                                                                            end: 646,
                                                                                            as_str(): "__size_of",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: Some(
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 646,
                                                                                                    end: 648,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            GenericArgs {
                                                                                                parameters: AngleBrackets {
                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                            ),
                                                                                                            start: 648,
                                                                                                            end: 649,
                                                                                                            as_str(): "<",
                                                                                                        },
                                                                                                    },
                                                                                                    inner: Punctuated {
                                                                                                        value_separator_pairs: [],
                                                                                                        final_value_opt: Some(
                                                                                                            Path(
                                                                                                                PathType {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathTypeSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 649,
                                                                                                                                end: 653,
                                                                                                                                as_str(): "Data",
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
                                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                            ),
                                                                                                            start: 653,
                                                                                                            end: 654,
                                                                                                            as_str(): ">",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 654,
                                                                                end: 656,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 657,
                                                                            end: 659,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 660,
                                                                                    end: 662,
                                                                                    as_str(): "24",
                                                                                },
                                                                                parsed: 24,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 636,
                                                            end: 663,
                                                            as_str(): "(__size_of::<Data>() == 24)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 663,
                                                            end: 664,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 669,
                                                                        end: 675,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 676,
                                                                                            end: 689,
                                                                                            as_str(): "__size_of_val",
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
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                        ),
                                                                                                        start: 690,
                                                                                                        end: 691,
                                                                                                        as_str(): "p",
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
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 689,
                                                                                end: 692,
                                                                                as_str(): "(p)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 693,
                                                                            end: 695,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 696,
                                                                                    end: 698,
                                                                                    as_str(): "24",
                                                                                },
                                                                                parsed: 24,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 675,
                                                            end: 699,
                                                            as_str(): "(__size_of_val(p) == 24)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 699,
                                                            end: 700,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 705,
                                                                        end: 711,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 712,
                                                                                            end: 725,
                                                                                            as_str(): "__size_of_val",
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
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04328bd30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                        ),
                                                                                                        start: 726,
                                                                                                        end: 729,
                                                                                                        as_str(): "foo",
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
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 725,
                                                                                end: 730,
                                                                                as_str(): "(foo)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 731,
                                                                            end: 733,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 734,
                                                                                    end: 735,
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
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 711,
                                                            end: 736,
                                                            as_str(): "(__size_of_val(foo) == 8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 736,
                                                            end: 737,
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
                                                                        src (ptr): 0x00007fe04328bd30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                        ),
                                                                        start: 742,
                                                                        end: 748,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                            ),
                                                                                            start: 749,
                                                                                            end: 758,
                                                                                            as_str(): "__size_of",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: Some(
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                    ),
                                                                                                    start: 758,
                                                                                                    end: 760,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            GenericArgs {
                                                                                                parameters: AngleBrackets {
                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                            ),
                                                                                                            start: 760,
                                                                                                            end: 761,
                                                                                                            as_str(): "<",
                                                                                                        },
                                                                                                    },
                                                                                                    inner: Punctuated {
                                                                                                        value_separator_pairs: [],
                                                                                                        final_value_opt: Some(
                                                                                                            Path(
                                                                                                                PathType {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathTypeSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 761,
                                                                                                                                end: 766,
                                                                                                                                as_str(): "Point",
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
                                                                                                            src (ptr): 0x00007fe04328bd30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                                            ),
                                                                                                            start: 766,
                                                                                                            end: 767,
                                                                                                            as_str(): ">",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        args: Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04328bd30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                ),
                                                                                start: 767,
                                                                                end: 769,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04328bd30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                            ),
                                                                            start: 770,
                                                                            end: 772,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04328bd30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                                                    ),
                                                                                    start: 773,
                                                                                    end: 775,
                                                                                    as_str(): "24",
                                                                                },
                                                                                parsed: 24,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 748,
                                                            end: 776,
                                                            as_str(): "(__size_of::<Point>() == 24)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 776,
                                                            end: 777,
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
                                                            src (ptr): 0x00007fe04328bd30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                                            ),
                                                            start: 782,
                                                            end: 783,
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
                                        src (ptr): 0x00007fe04328bd30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCrc1Hk/size_of/src/main.sw",
                                        ),
                                        start: 289,
                                        end: 785,
                                        as_str(): "{\n    let x = Data {\n        one: 1,\n        two: 2,\n        three: 3,\n    };\n    let y = Data {\n        one: 10000,\n        two: 20000,\n        three: 30000,\n    };\n    let p = Point {\n      x: 0,\n      y: 1,\n      z: 2\n    };\n    let foo = return_the_same(7u64);\n    assert(__size_of_val(x) == 24);\n    assert(__size_of_val(y) == 24);\n    assert(__size_of::<Data>() == 24);\n    assert(__size_of_val(p) == 24);\n    assert(__size_of_val(foo) == 8);\n    assert(__size_of::<Point>() == 24);\n    1\n}",
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
