Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb11f3db040,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb11f3db040,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 22,
                                                as_str(): "hash",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 22,
                                                end: 24,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 30,
                                                    as_str(): "sha256",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 39,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 40,
                                        end: 48,
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
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 56,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 56,
                                                                end: 57,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 58,
                                                                            end: 61,
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
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 61,
                                                        end: 62,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 68,
                                                            end: 69,
                                                            as_str(): "y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 69,
                                                            end: 70,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 71,
                                                                        end: 75,
                                                                        as_str(): "bool",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 49,
                                        end: 77,
                                        as_str(): "{\n    x: u64, \n    y: bool\n}",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 79,
                                        end: 83,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 84,
                                        end: 90,
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
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 97,
                                                                end: 98,
                                                                as_str(): "A",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 98,
                                                                end: 99,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 103,
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
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 103,
                                                        end: 104,
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
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 109,
                                                                end: 110,
                                                                as_str(): "B",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 111,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 116,
                                                                            as_str(): "bool",
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
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 116,
                                                        end: 117,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 91,
                                        end: 119,
                                        as_str(): "{\n    A: u64,\n    B: bool,\n}",
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
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 121,
                                        end: 125,
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
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 126,
                                                        end: 130,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 132,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 132,
                                                                end: 135,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 137,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 137,
                                                                end: 139,
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
                                                src (ptr): 0x00007fb11f3db040,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                ),
                                                start: 140,
                                                end: 143,
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
                                                    src (ptr): 0x00007fb11f3db040,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 150,
                                                    as_str(): "MyEnum",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 159,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 160,
                                                            end: 162,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 176,
                                                                                                    end: 182,
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
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 162,
                                                            end: 183,
                                                            as_str(): "(self, other: MyEnum)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 184,
                                                                    end: 186,
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 187,
                                                                                end: 191,
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
                                                            Match {
                                                                match_token: MatchToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 202,
                                                                        end: 207,
                                                                        as_str(): "match",
                                                                    },
                                                                },
                                                                value: Tuple(
                                                                    Parens {
                                                                        inner: Cons {
                                                                            head: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 209,
                                                                                                end: 213,
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
                                                                            comma_token: CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 213,
                                                                                    end: 214,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                            tail: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 215,
                                                                                                        end: 220,
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
                                                                            },
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 208,
                                                                            end: 221,
                                                                            as_str(): "(self, other)",
                                                                        },
                                                                    },
                                                                ),
                                                                branches: Braces {
                                                                    inner: [
                                                                        MatchBranch {
                                                                            pattern: Tuple(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                Constructor {
                                                                                                    path: PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 237,
                                                                                                                    end: 243,
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
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 243,
                                                                                                                        end: 245,
                                                                                                                        as_str(): "::",
                                                                                                                    },
                                                                                                                },
                                                                                                                PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 245,
                                                                                                                            end: 246,
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
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 247,
                                                                                                                            end: 253,
                                                                                                                            as_str(): "inner1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 246,
                                                                                                            end: 254,
                                                                                                            as_str(): "(inner1)",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 254,
                                                                                                        end: 255,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            Constructor {
                                                                                                path: PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 256,
                                                                                                                end: 262,
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
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 262,
                                                                                                                    end: 264,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 264,
                                                                                                                        end: 265,
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
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 266,
                                                                                                                        end: 272,
                                                                                                                        as_str(): "inner2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 265,
                                                                                                        end: 273,
                                                                                                        as_str(): "(inner2)",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 236,
                                                                                        end: 274,
                                                                                        as_str(): "(MyEnum::A(inner1), MyEnum::A(inner2))",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 276,
                                                                                    end: 278,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Equal {
                                                                                    lhs: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 279,
                                                                                                        end: 285,
                                                                                                        as_str(): "inner1",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 286,
                                                                                            end: 288,
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 289,
                                                                                                        end: 295,
                                                                                                        as_str(): "inner2",
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
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 295,
                                                                                        end: 296,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            pattern: Tuple(
                                                                                Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                Constructor {
                                                                                                    path: PathExpr {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 310,
                                                                                                                    end: 316,
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
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 316,
                                                                                                                        end: 318,
                                                                                                                        as_str(): "::",
                                                                                                                    },
                                                                                                                },
                                                                                                                PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 318,
                                                                                                                            end: 319,
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
                                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 320,
                                                                                                                            end: 326,
                                                                                                                            as_str(): "inner1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                            ),
                                                                                                            start: 319,
                                                                                                            end: 327,
                                                                                                            as_str(): "(inner1)",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 327,
                                                                                                        end: 328,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            Constructor {
                                                                                                path: PathExpr {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                ),
                                                                                                                start: 329,
                                                                                                                end: 335,
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
                                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 335,
                                                                                                                    end: 337,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 337,
                                                                                                                        end: 338,
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
                                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 339,
                                                                                                                        end: 345,
                                                                                                                        as_str(): "inner2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 338,
                                                                                                        end: 346,
                                                                                                        as_str(): "(inner2)",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 309,
                                                                                        end: 347,
                                                                                        as_str(): "(MyEnum::B(inner1), MyEnum::B(inner2))",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 349,
                                                                                    end: 351,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Equal {
                                                                                    lhs: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 352,
                                                                                                        end: 358,
                                                                                                        as_str(): "inner1",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 359,
                                                                                            end: 361,
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 362,
                                                                                                        end: 368,
                                                                                                        as_str(): "inner2",
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
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 368,
                                                                                        end: 369,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            pattern: Wildcard {
                                                                                underscore_token: UnderscoreToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 382,
                                                                                        end: 383,
                                                                                        as_str(): "_",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 384,
                                                                                    end: 386,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 387,
                                                                                                end: 392,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                            kind: False,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                comma_token: CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 392,
                                                                                        end: 393,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 222,
                                                                        end: 403,
                                                                        as_str(): "{\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 192,
                                                        end: 409,
                                                        as_str(): "{\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 411,
                                        as_str(): "{\n    fn eq(self, other: MyEnum) -> bool {\n        match (self, other) {\n            (MyEnum::A(inner1), MyEnum::A(inner2))  => inner1 == inner2,\n            (MyEnum::B(inner1), MyEnum::B(inner2))  => inner1 == inner2,\n            _ => false,\n        }\n    }\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Configurable(
                            ItemConfigurable {
                                configurable_token: ConfigurableToken {
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 413,
                                        end: 425,
                                        as_str(): "configurable",
                                    },
                                },
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 432,
                                                                end: 434,
                                                                as_str(): "C0",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 434,
                                                                end: 435,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 436,
                                                                            end: 440,
                                                                            as_str(): "bool",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 441,
                                                                end: 442,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Literal(
                                                            Bool(
                                                                LitBool {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 443,
                                                                        end: 447,
                                                                        as_str(): "true",
                                                                    },
                                                                    kind: True,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 447,
                                                        end: 448,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 453,
                                                                end: 455,
                                                                as_str(): "C1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 455,
                                                                end: 456,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 457,
                                                                            end: 460,
                                                                            as_str(): "u64",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 461,
                                                                end: 462,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 463,
                                                                        end: 465,
                                                                        as_str(): "42",
                                                                    },
                                                                    parsed: 42,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 465,
                                                        end: 466,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 471,
                                                                end: 473,
                                                                as_str(): "C2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 473,
                                                                end: 474,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 475,
                                                                            end: 479,
                                                                            as_str(): "b256",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 480,
                                                                end: 481,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 482,
                                                                        end: 548,
                                                                        as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                                    },
                                                                    parsed: 7719472615821079694904732333912527190217998977709370935963838933860875309329,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 548,
                                                        end: 549,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 554,
                                                                end: 556,
                                                                as_str(): "C3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 556,
                                                                end: 557,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 558,
                                                                            end: 566,
                                                                            as_str(): "MyStruct",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 567,
                                                                end: 568,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Struct {
                                                            path: PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 569,
                                                                            end: 577,
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
                                                                    value_separator_pairs: [
                                                                        (
                                                                            ExprStructField {
                                                                                field_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 580,
                                                                                        end: 581,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                expr_opt: Some(
                                                                                    (
                                                                                        ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 581,
                                                                                                end: 582,
                                                                                                as_str(): ":",
                                                                                            },
                                                                                        },
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 583,
                                                                                                        end: 585,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 585,
                                                                                    end: 586,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 587,
                                                                                    end: 588,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 588,
                                                                                            end: 589,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 590,
                                                                                                    end: 594,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 578,
                                                                    end: 596,
                                                                    as_str(): "{ x: 42, y: true }",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 596,
                                                        end: 597,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 602,
                                                                end: 604,
                                                                as_str(): "C4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 604,
                                                                end: 605,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 606,
                                                                            end: 612,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 613,
                                                                end: 614,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: FuncApp {
                                                            func: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 615,
                                                                                end: 621,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 621,
                                                                                    end: 623,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 623,
                                                                                        end: 624,
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
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 625,
                                                                                        end: 627,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                    parsed: 42,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 624,
                                                                    end: 628,
                                                                    as_str(): "(42)",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 628,
                                                        end: 629,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 634,
                                                                end: 636,
                                                                as_str(): "C5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 636,
                                                                end: 637,
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 638,
                                                                            end: 644,
                                                                            as_str(): "MyEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 645,
                                                                end: 646,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: FuncApp {
                                                            func: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 647,
                                                                                end: 653,
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 653,
                                                                                    end: 655,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 655,
                                                                                        end: 656,
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
                                                                        Literal(
                                                                            Bool(
                                                                                LitBool {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 657,
                                                                                        end: 661,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                    kind: True,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 656,
                                                                    end: 662,
                                                                    as_str(): "(true)",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 662,
                                                        end: 663,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 668,
                                                                end: 670,
                                                                as_str(): "C6",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 670,
                                                                end: 671,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 672,
                                                                    end: 675,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 676,
                                                                                end: 677,
                                                                                as_str(): "4",
                                                                            },
                                                                            parsed: 4,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 675,
                                                                    end: 678,
                                                                    as_str(): "[4]",
                                                                },
                                                            },
                                                        },
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 679,
                                                                end: 680,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Literal(
                                                            String(
                                                                LitString {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 681,
                                                                        end: 687,
                                                                        as_str(): "\"fuel\"",
                                                                    },
                                                                    parsed: "fuel",
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 687,
                                                        end: 688,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: ConfigurableField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 693,
                                                                end: 695,
                                                                as_str(): "C7",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 695,
                                                                end: 696,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Array(
                                                            SquareBrackets {
                                                                inner: TyArrayDescriptor {
                                                                    ty: Path(
                                                                        PathType {
                                                                            root_opt: None,
                                                                            prefix: PathTypeSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 698,
                                                                                        end: 701,
                                                                                        as_str(): "u64",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                        },
                                                                    ),
                                                                    semicolon_token: SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 701,
                                                                            end: 702,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                    length: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 703,
                                                                                    end: 704,
                                                                                    as_str(): "4",
                                                                                },
                                                                                parsed: 4,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 697,
                                                                    end: 705,
                                                                    as_str(): "[u64; 4]",
                                                                },
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb11f3db040,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                ),
                                                                start: 706,
                                                                end: 707,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Array(
                                                            SquareBrackets {
                                                                inner: Sequence(
                                                                    Punctuated {
                                                                        value_separator_pairs: [
                                                                            (
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 709,
                                                                                                end: 710,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 710,
                                                                                        end: 711,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 712,
                                                                                                end: 713,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                            parsed: 2,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 713,
                                                                                        end: 714,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 715,
                                                                                                end: 716,
                                                                                                as_str(): "3",
                                                                                            },
                                                                                            parsed: 3,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 716,
                                                                                        end: 717,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 718,
                                                                                            end: 719,
                                                                                            as_str(): "4",
                                                                                        },
                                                                                        parsed: 4,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb11f3db040,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                    ),
                                                                    start: 708,
                                                                    end: 720,
                                                                    as_str(): "[1, 2, 3, 4]",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 720,
                                                        end: 721,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 426,
                                        end: 723,
                                        as_str(): "{\n    C0: bool = true,\n    C1: u64 = 42,\n    C2: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111,\n    C3: MyStruct = MyStruct { x: 42, y: true },\n    C4: MyEnum = MyEnum::A(42),\n    C5: MyEnum = MyEnum::B(true),\n    C6: str[4] = \"fuel\",\n    C7: [u64; 4] = [1, 2, 3, 4],\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 725,
                                        end: 726,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 727,
                                                        end: 733,
                                                        as_str(): "inline",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 734,
                                                                        end: 739,
                                                                        as_str(): "never",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 733,
                                                            end: 740,
                                                            as_str(): "(never)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 726,
                                        end: 741,
                                        as_str(): "[inline(never)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 742,
                                            end: 744,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 745,
                                            end: 759,
                                            as_str(): "test_first_use",
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
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 759,
                                            end: 761,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 768,
                                                                        end: 774,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 775,
                                                                                        end: 777,
                                                                                        as_str(): "C0",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 778,
                                                                            end: 780,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 781,
                                                                                    end: 785,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 774,
                                                            end: 786,
                                                            as_str(): "(C0 == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 786,
                                                            end: 787,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 792,
                                                                        end: 798,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 799,
                                                                                        end: 801,
                                                                                        as_str(): "C1",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 802,
                                                                            end: 804,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 805,
                                                                                    end: 807,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 798,
                                                            end: 808,
                                                            as_str(): "(C1 == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 808,
                                                            end: 809,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 814,
                                                                        end: 820,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 821,
                                                                                        end: 823,
                                                                                        as_str(): "C2",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 824,
                                                                            end: 826,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 827,
                                                                                    end: 893,
                                                                                    as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                                                },
                                                                                parsed: 7719472615821079694904732333912527190217998977709370935963838933860875309329,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 820,
                                                            end: 894,
                                                            as_str(): "(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 894,
                                                            end: 895,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 900,
                                                                        end: 906,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 907,
                                                                                            end: 909,
                                                                                            as_str(): "C3",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 909,
                                                                                end: 910,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 910,
                                                                                end: 911,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 912,
                                                                            end: 914,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 915,
                                                                                    end: 917,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 906,
                                                            end: 918,
                                                            as_str(): "(C3.x == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 918,
                                                            end: 919,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 924,
                                                                        end: 930,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 931,
                                                                                            end: 933,
                                                                                            as_str(): "C3",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 933,
                                                                                end: 934,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 934,
                                                                                end: 935,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 936,
                                                                            end: 938,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 939,
                                                                                    end: 943,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 930,
                                                            end: 944,
                                                            as_str(): "(C3.y == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 944,
                                                            end: 945,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 950,
                                                                        end: 956,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 957,
                                                                                        end: 959,
                                                                                        as_str(): "C4",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 960,
                                                                            end: 962,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 963,
                                                                                            end: 969,
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
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 969,
                                                                                                end: 971,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 971,
                                                                                                    end: 972,
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 973,
                                                                                                    end: 975,
                                                                                                    as_str(): "42",
                                                                                                },
                                                                                                parsed: 42,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 972,
                                                                                end: 976,
                                                                                as_str(): "(42)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 956,
                                                            end: 977,
                                                            as_str(): "(C4 == MyEnum::A(42))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 977,
                                                            end: 978,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 983,
                                                                        end: 989,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 990,
                                                                                        end: 992,
                                                                                        as_str(): "C5",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 993,
                                                                            end: 995,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 996,
                                                                                            end: 1002,
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
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1002,
                                                                                                end: 1004,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1004,
                                                                                                    end: 1005,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1006,
                                                                                                    end: 1010,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1005,
                                                                                end: 1011,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 989,
                                                            end: 1012,
                                                            as_str(): "(C5 == MyEnum::B(true))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1012,
                                                            end: 1013,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1018,
                                                                        end: 1024,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1025,
                                                                                            end: 1031,
                                                                                            as_str(): "sha256",
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1032,
                                                                                                        end: 1034,
                                                                                                        as_str(): "C6",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1031,
                                                                                end: 1035,
                                                                                as_str(): "(C6)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1036,
                                                                            end: 1038,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1039,
                                                                                            end: 1045,
                                                                                            as_str(): "sha256",
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
                                                                                        String(
                                                                                            LitString {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1046,
                                                                                                    end: 1052,
                                                                                                    as_str(): "\"fuel\"",
                                                                                                },
                                                                                                parsed: "fuel",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1045,
                                                                                end: 1053,
                                                                                as_str(): "(\"fuel\")",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1054,
                                                            as_str(): "(sha256(C6) == sha256(\"fuel\"))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1054,
                                                            end: 1055,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1060,
                                                                        end: 1066,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1067,
                                                                                            end: 1069,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1070,
                                                                                            end: 1071,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                        parsed: 0,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1069,
                                                                                end: 1072,
                                                                                as_str(): "[0]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1073,
                                                                            end: 1075,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1076,
                                                                                    end: 1077,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1066,
                                                            end: 1078,
                                                            as_str(): "(C7[0] == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1078,
                                                            end: 1079,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1084,
                                                                        end: 1090,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1091,
                                                                                            end: 1093,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1094,
                                                                                            end: 1095,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1093,
                                                                                end: 1096,
                                                                                as_str(): "[1]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1100,
                                                                                    end: 1101,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1090,
                                                            end: 1102,
                                                            as_str(): "(C7[1] == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1115,
                                                                                            end: 1117,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1118,
                                                                                            end: 1119,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1117,
                                                                                end: 1120,
                                                                                as_str(): "[2]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1121,
                                                                            end: 1123,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1124,
                                                                                    end: 1125,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1114,
                                                            end: 1126,
                                                            as_str(): "(C7[2] == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1126,
                                                            end: 1127,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1132,
                                                                        end: 1138,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1139,
                                                                                            end: 1141,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1142,
                                                                                            end: 1143,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                        parsed: 3,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1141,
                                                                                end: 1144,
                                                                                as_str(): "[3]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1145,
                                                                            end: 1147,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1148,
                                                                                    end: 1149,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1138,
                                                            end: 1150,
                                                            as_str(): "(C7[3] == 4)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1150,
                                                            end: 1151,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 762,
                                        end: 1153,
                                        as_str(): "{\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1155,
                                        end: 1156,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1157,
                                                        end: 1163,
                                                        as_str(): "inline",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1164,
                                                                        end: 1169,
                                                                        as_str(): "never",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1163,
                                                            end: 1170,
                                                            as_str(): "(never)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1156,
                                        end: 1171,
                                        as_str(): "[inline(never)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1172,
                                            end: 1174,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1175,
                                            end: 1190,
                                            as_str(): "test_second_use",
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
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1190,
                                            end: 1192,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1199,
                                                                        end: 1205,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1206,
                                                                                        end: 1208,
                                                                                        as_str(): "C0",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1209,
                                                                            end: 1211,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1212,
                                                                                    end: 1216,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1205,
                                                            end: 1217,
                                                            as_str(): "(C0 == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1217,
                                                            end: 1218,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1223,
                                                                        end: 1229,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1230,
                                                                                        end: 1232,
                                                                                        as_str(): "C1",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1233,
                                                                            end: 1235,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1236,
                                                                                    end: 1238,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1229,
                                                            end: 1239,
                                                            as_str(): "(C1 == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1239,
                                                            end: 1240,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1245,
                                                                        end: 1251,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1252,
                                                                                        end: 1254,
                                                                                        as_str(): "C2",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1255,
                                                                            end: 1257,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1258,
                                                                                    end: 1324,
                                                                                    as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                                                },
                                                                                parsed: 7719472615821079694904732333912527190217998977709370935963838933860875309329,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1251,
                                                            end: 1325,
                                                            as_str(): "(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1325,
                                                            end: 1326,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1331,
                                                                        end: 1337,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1338,
                                                                                            end: 1340,
                                                                                            as_str(): "C3",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1340,
                                                                                end: 1341,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1341,
                                                                                end: 1342,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1343,
                                                                            end: 1345,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1346,
                                                                                    end: 1348,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1337,
                                                            end: 1349,
                                                            as_str(): "(C3.x == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1349,
                                                            end: 1350,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1355,
                                                                        end: 1361,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1362,
                                                                                            end: 1364,
                                                                                            as_str(): "C3",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1364,
                                                                                end: 1365,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1365,
                                                                                end: 1366,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1367,
                                                                            end: 1369,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1370,
                                                                                    end: 1374,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1361,
                                                            end: 1375,
                                                            as_str(): "(C3.y == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1375,
                                                            end: 1376,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1381,
                                                                        end: 1387,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1388,
                                                                                        end: 1390,
                                                                                        as_str(): "C4",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1391,
                                                                            end: 1393,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1394,
                                                                                            end: 1400,
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
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1400,
                                                                                                end: 1402,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1402,
                                                                                                    end: 1403,
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1404,
                                                                                                    end: 1406,
                                                                                                    as_str(): "42",
                                                                                                },
                                                                                                parsed: 42,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1403,
                                                                                end: 1407,
                                                                                as_str(): "(42)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1387,
                                                            end: 1408,
                                                            as_str(): "(C4 == MyEnum::A(42))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1408,
                                                            end: 1409,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1414,
                                                                        end: 1420,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1421,
                                                                                        end: 1423,
                                                                                        as_str(): "C5",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1424,
                                                                            end: 1426,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1427,
                                                                                            end: 1433,
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
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1433,
                                                                                                end: 1435,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1435,
                                                                                                    end: 1436,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1437,
                                                                                                    end: 1441,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1436,
                                                                                end: 1442,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1420,
                                                            end: 1443,
                                                            as_str(): "(C5 == MyEnum::B(true))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1443,
                                                            end: 1444,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1449,
                                                                        end: 1455,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1456,
                                                                                            end: 1462,
                                                                                            as_str(): "sha256",
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1463,
                                                                                                        end: 1465,
                                                                                                        as_str(): "C6",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1462,
                                                                                end: 1466,
                                                                                as_str(): "(C6)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1467,
                                                                            end: 1469,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1470,
                                                                                            end: 1476,
                                                                                            as_str(): "sha256",
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
                                                                                        String(
                                                                                            LitString {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1477,
                                                                                                    end: 1483,
                                                                                                    as_str(): "\"fuel\"",
                                                                                                },
                                                                                                parsed: "fuel",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1476,
                                                                                end: 1484,
                                                                                as_str(): "(\"fuel\")",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1455,
                                                            end: 1485,
                                                            as_str(): "(sha256(C6) == sha256(\"fuel\"))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1485,
                                                            end: 1486,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1491,
                                                                        end: 1497,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1498,
                                                                                            end: 1500,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1501,
                                                                                            end: 1502,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                        parsed: 0,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1500,
                                                                                end: 1503,
                                                                                as_str(): "[0]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1504,
                                                                            end: 1506,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1507,
                                                                                    end: 1508,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1497,
                                                            end: 1509,
                                                            as_str(): "(C7[0] == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1509,
                                                            end: 1510,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1515,
                                                                        end: 1521,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1522,
                                                                                            end: 1524,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1525,
                                                                                            end: 1526,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1524,
                                                                                end: 1527,
                                                                                as_str(): "[1]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1528,
                                                                            end: 1530,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1531,
                                                                                    end: 1532,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1521,
                                                            end: 1533,
                                                            as_str(): "(C7[1] == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1533,
                                                            end: 1534,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1539,
                                                                        end: 1545,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1546,
                                                                                            end: 1548,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1549,
                                                                                            end: 1550,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1548,
                                                                                end: 1551,
                                                                                as_str(): "[2]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1552,
                                                                            end: 1554,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1555,
                                                                                    end: 1556,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1545,
                                                            end: 1557,
                                                            as_str(): "(C7[2] == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1557,
                                                            end: 1558,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1563,
                                                                        end: 1569,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1570,
                                                                                            end: 1572,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1573,
                                                                                            end: 1574,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                        parsed: 3,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1572,
                                                                                end: 1575,
                                                                                as_str(): "[3]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1576,
                                                                            end: 1578,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1579,
                                                                                    end: 1580,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1569,
                                                            end: 1581,
                                                            as_str(): "(C7[3] == 4)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1581,
                                                            end: 1582,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1193,
                                        end: 1584,
                                        as_str(): "{\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1586,
                                        end: 1587,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 1588,
                                                        end: 1594,
                                                        as_str(): "inline",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1595,
                                                                        end: 1601,
                                                                        as_str(): "always",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1594,
                                                            end: 1602,
                                                            as_str(): "(always)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1587,
                                        end: 1603,
                                        as_str(): "[inline(always)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1604,
                                            end: 1606,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1607,
                                            end: 1622,
                                            as_str(): "test_inline_use",
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
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 1622,
                                            end: 1624,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1631,
                                                                        end: 1637,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1638,
                                                                                        end: 1640,
                                                                                        as_str(): "C0",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1641,
                                                                            end: 1643,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1644,
                                                                                    end: 1648,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1637,
                                                            end: 1649,
                                                            as_str(): "(C0 == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1649,
                                                            end: 1650,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1655,
                                                                        end: 1661,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1662,
                                                                                        end: 1664,
                                                                                        as_str(): "C1",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1665,
                                                                            end: 1667,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1668,
                                                                                    end: 1670,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1661,
                                                            end: 1671,
                                                            as_str(): "(C1 == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1671,
                                                            end: 1672,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1677,
                                                                        end: 1683,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1684,
                                                                                        end: 1686,
                                                                                        as_str(): "C2",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1687,
                                                                            end: 1689,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1690,
                                                                                    end: 1756,
                                                                                    as_str(): "0x1111111111111111111111111111111111111111111111111111111111111111",
                                                                                },
                                                                                parsed: 7719472615821079694904732333912527190217998977709370935963838933860875309329,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1683,
                                                            end: 1757,
                                                            as_str(): "(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1757,
                                                            end: 1758,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1763,
                                                                        end: 1769,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1770,
                                                                                            end: 1772,
                                                                                            as_str(): "C3",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1772,
                                                                                end: 1773,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1773,
                                                                                end: 1774,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1775,
                                                                            end: 1777,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1778,
                                                                                    end: 1780,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1769,
                                                            end: 1781,
                                                            as_str(): "(C3.x == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1781,
                                                            end: 1782,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1787,
                                                                        end: 1793,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1794,
                                                                                            end: 1796,
                                                                                            as_str(): "C3",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1796,
                                                                                end: 1797,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1797,
                                                                                end: 1798,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1799,
                                                                            end: 1801,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1802,
                                                                                    end: 1806,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1793,
                                                            end: 1807,
                                                            as_str(): "(C3.y == true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1807,
                                                            end: 1808,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1813,
                                                                        end: 1819,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1820,
                                                                                        end: 1822,
                                                                                        as_str(): "C4",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1823,
                                                                            end: 1825,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1826,
                                                                                            end: 1832,
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
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1832,
                                                                                                end: 1834,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1834,
                                                                                                    end: 1835,
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1836,
                                                                                                    end: 1838,
                                                                                                    as_str(): "42",
                                                                                                },
                                                                                                parsed: 42,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1835,
                                                                                end: 1839,
                                                                                as_str(): "(42)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1819,
                                                            end: 1840,
                                                            as_str(): "(C4 == MyEnum::A(42))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1840,
                                                            end: 1841,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1846,
                                                                        end: 1852,
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
                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                        ),
                                                                                        start: 1853,
                                                                                        end: 1855,
                                                                                        as_str(): "C5",
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
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1856,
                                                                            end: 1858,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1859,
                                                                                            end: 1865,
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
                                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                ),
                                                                                                start: 1865,
                                                                                                end: 1867,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1867,
                                                                                                    end: 1868,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1869,
                                                                                                    end: 1873,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1868,
                                                                                end: 1874,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1852,
                                                            end: 1875,
                                                            as_str(): "(C5 == MyEnum::B(true))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1875,
                                                            end: 1876,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1881,
                                                                        end: 1887,
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1888,
                                                                                            end: 1894,
                                                                                            as_str(): "sha256",
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
                                                                                                        src (ptr): 0x00007fb11f3db040,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1895,
                                                                                                        end: 1897,
                                                                                                        as_str(): "C6",
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
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1894,
                                                                                end: 1898,
                                                                                as_str(): "(C6)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1899,
                                                                            end: 1901,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1902,
                                                                                            end: 1908,
                                                                                            as_str(): "sha256",
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
                                                                                        String(
                                                                                            LitString {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1909,
                                                                                                    end: 1915,
                                                                                                    as_str(): "\"fuel\"",
                                                                                                },
                                                                                                parsed: "fuel",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1908,
                                                                                end: 1916,
                                                                                as_str(): "(\"fuel\")",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1887,
                                                            end: 1917,
                                                            as_str(): "(sha256(C6) == sha256(\"fuel\"))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1917,
                                                            end: 1918,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1923,
                                                                        end: 1929,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1930,
                                                                                            end: 1932,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1933,
                                                                                            end: 1934,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                        parsed: 0,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1932,
                                                                                end: 1935,
                                                                                as_str(): "[0]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1936,
                                                                            end: 1938,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1939,
                                                                                    end: 1940,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1929,
                                                            end: 1941,
                                                            as_str(): "(C7[0] == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1941,
                                                            end: 1942,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1947,
                                                                        end: 1953,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1954,
                                                                                            end: 1956,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1957,
                                                                                            end: 1958,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1956,
                                                                                end: 1959,
                                                                                as_str(): "[1]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1960,
                                                                            end: 1962,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1963,
                                                                                    end: 1964,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1953,
                                                            end: 1965,
                                                            as_str(): "(C7[1] == 2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1965,
                                                            end: 1966,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1971,
                                                                        end: 1977,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1978,
                                                                                            end: 1980,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 1981,
                                                                                            end: 1982,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 1980,
                                                                                end: 1983,
                                                                                as_str(): "[2]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 1984,
                                                                            end: 1986,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 1987,
                                                                                    end: 1988,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1977,
                                                            end: 1989,
                                                            as_str(): "(C7[2] == 3)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 1989,
                                                            end: 1990,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 1995,
                                                                        end: 2001,
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
                                                                    lhs: Index {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 2002,
                                                                                            end: 2004,
                                                                                            as_str(): "C7",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        arg: SquareBrackets {
                                                                            inner: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb11f3db040,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                            ),
                                                                                            start: 2005,
                                                                                            end: 2006,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                        parsed: 3,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb11f3db040,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                ),
                                                                                start: 2004,
                                                                                end: 2007,
                                                                                as_str(): "[3]",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb11f3db040,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                            ),
                                                                            start: 2008,
                                                                            end: 2010,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb11f3db040,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                                    ),
                                                                                    start: 2011,
                                                                                    end: 2012,
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2001,
                                                            end: 2013,
                                                            as_str(): "(C7[3] == 4)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2013,
                                                            end: 2014,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 1625,
                                        end: 2016,
                                        as_str(): "{\n    assert(C0 == true);\n    assert(C1 == 42);\n    assert(C2 == 0x1111111111111111111111111111111111111111111111111111111111111111);\n    assert(C3.x == 42);\n    assert(C3.y == true);\n    assert(C4 == MyEnum::A(42));\n    assert(C5 == MyEnum::B(true));\n    assert(sha256(C6) == sha256(\"fuel\"));\n    assert(C7[0] == 1);\n    assert(C7[1] == 2);\n    assert(C7[2] == 3);\n    assert(C7[3] == 4);\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2018,
                                        end: 2019,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb11f3db040,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                        ),
                                                        start: 2020,
                                                        end: 2026,
                                                        as_str(): "inline",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 2027,
                                                                        end: 2032,
                                                                        as_str(): "never",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2026,
                                                            end: 2033,
                                                            as_str(): "(never)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2019,
                                        end: 2034,
                                        as_str(): "[inline(never)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2035,
                                            end: 2037,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2038,
                                            end: 2055,
                                            as_str(): "test_various_uses",
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
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2055,
                                            end: 2057,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 2064,
                                                                        end: 2078,
                                                                        as_str(): "test_first_use",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2078,
                                                            end: 2080,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2080,
                                                            end: 2081,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 2086,
                                                                        end: 2101,
                                                                        as_str(): "test_second_use",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2101,
                                                            end: 2103,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2103,
                                                            end: 2104,
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
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 2109,
                                                                        end: 2124,
                                                                        as_str(): "test_inline_use",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2124,
                                                            end: 2126,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2126,
                                                            end: 2127,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2058,
                                        end: 2129,
                                        as_str(): "{\n    test_first_use();\n    test_second_use();\n    test_inline_use();\n}",
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
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2131,
                                            end: 2133,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2134,
                                            end: 2138,
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
                                            src (ptr): 0x00007fb11f3db040,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                            ),
                                            start: 2138,
                                            end: 2140,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb11f3db040,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                                        ),
                                                                        start: 2147,
                                                                        end: 2164,
                                                                        as_str(): "test_various_uses",
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
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2164,
                                                            end: 2166,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb11f3db040,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                                            ),
                                                            start: 2166,
                                                            end: 2167,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb11f3db040,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRp4YwIY/configurable_consts/src/main.sw",
                                        ),
                                        start: 2141,
                                        end: 2169,
                                        as_str(): "{\n    test_various_uses();\n}",
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
