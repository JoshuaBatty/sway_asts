Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0f2710960,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0f2710960,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                src (ptr): 0x00007fb0f2710960,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb0f2710960,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 41,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 42,
                                        end: 44,
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 53,
                                                                as_str(): "t1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 58,
                                                        end: 59,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 62,
                                        as_str(): "{\n    t1: u64, \n}",
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 70,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 71,
                                        end: 73,
                                        as_str(): "T2",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 80,
                                                                end: 82,
                                                                as_str(): "t1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 82,
                                                                end: 83,
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
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 84,
                                                                            end: 87,
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
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 87,
                                                        end: 88,
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
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 96,
                                                            as_str(): "t2",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 98,
                                                                        end: 101,
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 74,
                                        end: 103,
                                        as_str(): "{\n    t1: u64, \n    t2: u64\n}",
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 105,
                                        end: 109,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 110,
                                        end: 111,
                                        as_str(): "A",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 119,
                                                                as_str(): "A",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 119,
                                                                end: 120,
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
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 121,
                                                                            end: 124,
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
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 130,
                                                                end: 131,
                                                                as_str(): "B",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 131,
                                                                end: 132,
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
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 133,
                                                                            end: 135,
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
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 135,
                                                        end: 136,
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 141,
                                                                end: 142,
                                                                as_str(): "C",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 142,
                                                                end: 143,
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
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 146,
                                                                            as_str(): "T2",
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
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 146,
                                                        end: 147,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 112,
                                        end: 149,
                                        as_str(): "{\n    A: u64,\n    B: T1,\n    C: T2,\n}",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 151,
                                            end: 153,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 154,
                                            end: 158,
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 158,
                                            end: 160,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 161,
                                                    end: 163,
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 164,
                                                                end: 167,
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
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 174,
                                                            end: 177,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 178,
                                                                end: 179,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 180,
                                                            end: 181,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 182,
                                                                    end: 184,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Let {
                                                                let_token: LetToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 185,
                                                                        end: 188,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 189,
                                                                                    end: 190,
                                                                                    as_str(): "A",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 190,
                                                                                        end: 192,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 192,
                                                                                            end: 193,
                                                                                            as_str(): "A",
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
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 194,
                                                                                            end: 195,
                                                                                            as_str(): "n",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 193,
                                                                            end: 196,
                                                                            as_str(): "(n)",
                                                                        },
                                                                    },
                                                                },
                                                                eq_token: EqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 197,
                                                                        end: 198,
                                                                        as_str(): "=",
                                                                    },
                                                                },
                                                                rhs: FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 199,
                                                                                        end: 200,
                                                                                        as_str(): "A",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 200,
                                                                                            end: 202,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 202,
                                                                                                end: 203,
                                                                                                as_str(): "A",
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
                                                                                FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                        ),
                                                                                                        start: 204,
                                                                                                        end: 205,
                                                                                                        as_str(): "f",
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
                                                                                            final_value_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 205,
                                                                                            end: 207,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 203,
                                                                            end: 208,
                                                                            as_str(): "(f())",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            then_block: Braces {
                                                                inner: CodeBlockContents {
                                                                    statements: [],
                                                                    final_expr_opt: Some(
                                                                        Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 211,
                                                                                            end: 212,
                                                                                            as_str(): "n",
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
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 209,
                                                                    end: 214,
                                                                    as_str(): "{ n }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 215,
                                                                            end: 219,
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
                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                    ),
                                                                                                    start: 222,
                                                                                                    end: 223,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 220,
                                                                                end: 225,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 225,
                                                            end: 226,
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
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 231,
                                                                        end: 237,
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
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 238,
                                                                                        end: 239,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 240,
                                                                            end: 242,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 243,
                                                                                    end: 244,
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
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 237,
                                                            end: 245,
                                                            as_str(): "(x == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 245,
                                                            end: 246,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 252,
                                                            end: 255,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 256,
                                                                end: 257,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 258,
                                                            end: 259,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 260,
                                                                    end: 262,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Let {
                                                                let_token: LetToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 263,
                                                                        end: 266,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 267,
                                                                                    end: 268,
                                                                                    as_str(): "A",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 268,
                                                                                        end: 270,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 270,
                                                                                            end: 271,
                                                                                            as_str(): "B",
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
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 272,
                                                                                            end: 273,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 271,
                                                                            end: 274,
                                                                            as_str(): "(t)",
                                                                        },
                                                                    },
                                                                },
                                                                eq_token: EqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 275,
                                                                        end: 276,
                                                                        as_str(): "=",
                                                                    },
                                                                },
                                                                rhs: FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 277,
                                                                                        end: 278,
                                                                                        as_str(): "A",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 278,
                                                                                            end: 280,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 280,
                                                                                                end: 281,
                                                                                                as_str(): "B",
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
                                                                                FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                        ),
                                                                                                        start: 282,
                                                                                                        end: 283,
                                                                                                        as_str(): "g",
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
                                                                                            final_value_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 283,
                                                                                            end: 285,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 281,
                                                                            end: 286,
                                                                            as_str(): "(g())",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            then_block: Braces {
                                                                inner: CodeBlockContents {
                                                                    statements: [],
                                                                    final_expr_opt: Some(
                                                                        FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 289,
                                                                                                end: 290,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 290,
                                                                                    end: 291,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 291,
                                                                                    end: 293,
                                                                                    as_str(): "t1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 287,
                                                                    end: 295,
                                                                    as_str(): "{ t.t1 }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 296,
                                                                            end: 300,
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
                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                    ),
                                                                                                    start: 303,
                                                                                                    end: 304,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 301,
                                                                                end: 306,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 306,
                                                            end: 307,
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
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 312,
                                                                        end: 318,
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
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 319,
                                                                                        end: 320,
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
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 321,
                                                                            end: 323,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 324,
                                                                                    end: 326,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 318,
                                                            end: 327,
                                                            as_str(): "(y == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 327,
                                                            end: 328,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 334,
                                                            end: 337,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 338,
                                                                end: 339,
                                                                as_str(): "z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 341,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 342,
                                                                    end: 344,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Let {
                                                                let_token: LetToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 345,
                                                                        end: 348,
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
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 349,
                                                                                    end: 350,
                                                                                    as_str(): "A",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 350,
                                                                                        end: 352,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 352,
                                                                                            end: 353,
                                                                                            as_str(): "C",
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
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 354,
                                                                                            end: 355,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 353,
                                                                            end: 356,
                                                                            as_str(): "(t)",
                                                                        },
                                                                    },
                                                                },
                                                                eq_token: EqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 357,
                                                                        end: 358,
                                                                        as_str(): "=",
                                                                    },
                                                                },
                                                                rhs: FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 359,
                                                                                        end: 360,
                                                                                        as_str(): "A",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 360,
                                                                                            end: 362,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 362,
                                                                                                end: 363,
                                                                                                as_str(): "C",
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
                                                                                FuncApp {
                                                                                    func: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                        ),
                                                                                                        start: 364,
                                                                                                        end: 365,
                                                                                                        as_str(): "h",
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
                                                                                            final_value_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 365,
                                                                                            end: 367,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 363,
                                                                            end: 368,
                                                                            as_str(): "(h())",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            then_block: Braces {
                                                                inner: CodeBlockContents {
                                                                    statements: [],
                                                                    final_expr_opt: Some(
                                                                        FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                ),
                                                                                                start: 371,
                                                                                                end: 372,
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
                                                                            dot_token: DotToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 372,
                                                                                    end: 373,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 373,
                                                                                    end: 375,
                                                                                    as_str(): "t2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f2710960,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                    ),
                                                                    start: 369,
                                                                    end: 377,
                                                                    as_str(): "{ t.t2 }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 378,
                                                                            end: 382,
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
                                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                                    ),
                                                                                                    start: 385,
                                                                                                    end: 386,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 383,
                                                                                end: 388,
                                                                                as_str(): "{ 0 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 388,
                                                            end: 389,
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
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 394,
                                                                        end: 400,
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
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 401,
                                                                                        end: 402,
                                                                                        as_str(): "z",
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
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 403,
                                                                            end: 405,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 406,
                                                                                    end: 408,
                                                                                    as_str(): "66",
                                                                                },
                                                                                parsed: 66,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 400,
                                                            end: 409,
                                                            as_str(): "(z == 66)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 409,
                                                            end: 410,
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
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 416,
                                                            end: 417,
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 168,
                                        end: 419,
                                        as_str(): "{\n    let x = if let A::A(n) = A::A(f()) { n } else { 0 };\n    assert(x == 1);\n\n    let y = if let A::B(t) = A::B(g()) { t.t1 } else { 0 };\n    assert(y == 42);\n\n    let z = if let A::C(t) = A::C(h()) { t.t2 } else { 0 };\n    assert(z == 66);\n\n    1\n}",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 421,
                                            end: 423,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 424,
                                            end: 425,
                                            as_str(): "f",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 425,
                                            end: 427,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 428,
                                                    end: 430,
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 431,
                                                                end: 434,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f2710960,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                            ),
                                                            start: 441,
                                                            end: 442,
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
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 435,
                                        end: 444,
                                        as_str(): "{\n    1\n}",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 446,
                                            end: 448,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 449,
                                            end: 450,
                                            as_str(): "g",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 450,
                                            end: 452,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 453,
                                                    end: 455,
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 456,
                                                                end: 458,
                                                                as_str(): "T1",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 465,
                                                                end: 467,
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
                                                        value_separator_pairs: [],
                                                        final_value_opt: Some(
                                                            ExprStructField {
                                                                field_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 470,
                                                                        end: 472,
                                                                        as_str(): "t1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                expr_opt: Some(
                                                                    (
                                                                        ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 472,
                                                                                end: 473,
                                                                                as_str(): ":",
                                                                            },
                                                                        },
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 474,
                                                                                        end: 476,
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
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 468,
                                                        end: 478,
                                                        as_str(): "{ t1: 42 }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 459,
                                        end: 480,
                                        as_str(): "{\n    T1 { t1: 42 }\n}",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 482,
                                            end: 484,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 485,
                                            end: 486,
                                            as_str(): "h",
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
                                            src (ptr): 0x00007fb0f2710960,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                            ),
                                            start: 486,
                                            end: 488,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f2710960,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                    ),
                                                    start: 489,
                                                    end: 491,
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 492,
                                                                end: 494,
                                                                as_str(): "T2",
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
                                                                src (ptr): 0x00007fb0f2710960,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                ),
                                                                start: 501,
                                                                end: 503,
                                                                as_str(): "T2",
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
                                                                            src (ptr): 0x00007fb0f2710960,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                            ),
                                                                            start: 506,
                                                                            end: 508,
                                                                            as_str(): "t1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f2710960,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                    ),
                                                                                    start: 508,
                                                                                    end: 509,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f2710960,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                            ),
                                                                                            start: 510,
                                                                                            end: 512,
                                                                                            as_str(): "77",
                                                                                        },
                                                                                        parsed: 77,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 512,
                                                                        end: 513,
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
                                                                        src (ptr): 0x00007fb0f2710960,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                        ),
                                                                        start: 514,
                                                                        end: 516,
                                                                        as_str(): "t2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                expr_opt: Some(
                                                                    (
                                                                        ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f2710960,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                ),
                                                                                start: 516,
                                                                                end: 517,
                                                                                as_str(): ":",
                                                                            },
                                                                        },
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f2710960,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                                                        ),
                                                                                        start: 518,
                                                                                        end: 520,
                                                                                        as_str(): "66",
                                                                                    },
                                                                                    parsed: 66,
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
                                                        src (ptr): 0x00007fb0f2710960,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                                        ),
                                                        start: 504,
                                                        end: 522,
                                                        as_str(): "{ t1: 77, t2: 66 }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0f2710960,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlejRJh/enum_init_fn_call/src/main.sw",
                                        ),
                                        start: 495,
                                        end: 524,
                                        as_str(): "{\n    T2 { t1: 77, t2: 66 }\n}",
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
