Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0f97a4fa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0f97a4fa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 17,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 17,
                                            end: 19,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f97a4fa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 22,
                                                as_str(): "ops",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0f97a4fa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                ),
                                                start: 22,
                                                end: 24,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Glob {
                                            star_token: StarToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 24,
                                                    end: 25,
                                                    as_str(): "*",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 26,
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 30,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 31,
                                            end: 34,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 34,
                                            end: 36,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f97a4fa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                ),
                                                start: 36,
                                                end: 42,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0f97a4fa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                ),
                                                start: 42,
                                                end: 44,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Glob {
                                            star_token: StarToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 44,
                                                    end: 45,
                                                    as_str(): "*",
                                                },
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 46,
                                        as_str(): ";",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 52,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 60,
                                        as_str(): "Result2",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 60,
                                                    end: 61,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 62,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 62,
                                                                end: 63,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 64,
                                                            end: 65,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 65,
                                                    end: 66,
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
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 75,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 75,
                                                                end: 76,
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
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 77,
                                                                            end: 78,
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
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 78,
                                                        end: 79,
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
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 84,
                                                                end: 87,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 87,
                                                                end: 88,
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
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 89,
                                                                            end: 90,
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
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 67,
                                        end: 93,
                                        as_str(): "{\n    Ok: T,\n    Err: E,\n}",
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
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 95,
                                        end: 99,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 99,
                                                    end: 100,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 100,
                                                                end: 101,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 101,
                                                                end: 102,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 103,
                                                            end: 104,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 104,
                                                    end: 105,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
                                trait_opt: None,
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 106,
                                                    end: 113,
                                                    as_str(): "Result2",
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
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 113,
                                                                    end: 114,
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
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 114,
                                                                                            end: 115,
                                                                                            as_str(): "T",
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
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 115,
                                                                                end: 116,
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
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 117,
                                                                                        end: 118,
                                                                                        as_str(): "E",
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
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 118,
                                                                    end: 119,
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
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: Some(
                                                        PubToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 126,
                                                                end: 129,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 132,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 133,
                                                            end: 142,
                                                            as_str(): "unwrap_or",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 143,
                                                                    end: 147,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 147,
                                                                            end: 148,
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
                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                            ),
                                                                                            start: 149,
                                                                                            end: 156,
                                                                                            as_str(): "default",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 158,
                                                                                                    end: 159,
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 142,
                                                            end: 160,
                                                            as_str(): "(self, default: T)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
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
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 164,
                                                                                end: 165,
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
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Match {
                                                                match_token: MatchToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 176,
                                                                        end: 181,
                                                                        as_str(): "match",
                                                                    },
                                                                },
                                                                value: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 182,
                                                                                    end: 186,
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
                                                                branches: Braces {
                                                                    inner: [
                                                                        MatchBranch {
                                                                            pattern: Constructor {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 201,
                                                                                                end: 208,
                                                                                                as_str(): "Result2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 208,
                                                                                                    end: 210,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 210,
                                                                                                        end: 212,
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
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 213,
                                                                                                        end: 224,
                                                                                                        as_str(): "inner_value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 212,
                                                                                        end: 225,
                                                                                        as_str(): "(inner_value)",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 226,
                                                                                    end: 228,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 229,
                                                                                                    end: 240,
                                                                                                    as_str(): "inner_value",
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
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 240,
                                                                                        end: 241,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        MatchBranch {
                                                                            pattern: Constructor {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 254,
                                                                                                end: 261,
                                                                                                as_str(): "Result2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 261,
                                                                                                    end: 263,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 263,
                                                                                                        end: 266,
                                                                                                        as_str(): "Err",
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
                                                                                            Wildcard {
                                                                                                underscore_token: UnderscoreToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 267,
                                                                                                        end: 268,
                                                                                                        as_str(): "_",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 266,
                                                                                        end: 269,
                                                                                        as_str(): "(_)",
                                                                                    },
                                                                                },
                                                                            },
                                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 270,
                                                                                    end: 272,
                                                                                    as_str(): "=>",
                                                                                },
                                                                            },
                                                                            kind: Expr {
                                                                                expr: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 273,
                                                                                                    end: 280,
                                                                                                    as_str(): "default",
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
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 280,
                                                                                        end: 281,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 187,
                                                                        end: 291,
                                                                        as_str(): "{\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 166,
                                                        end: 297,
                                                        as_str(): "{\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 120,
                                        end: 299,
                                        as_str(): "{\n    pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result2::Ok(inner_value) => inner_value,\n            Result2::Err(_) => default,\n        }\n    }\n}",
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
                                    visibility: Some(
                                        PubToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0f97a4fa0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                ),
                                                start: 301,
                                                end: 304,
                                                as_str(): "pub",
                                            },
                                        },
                                    ),
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 305,
                                            end: 307,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 308,
                                            end: 322,
                                            as_str(): "test_unwrap_or",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 322,
                                                        end: 323,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 323,
                                                                end: 324,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                        ),
                                                        start: 324,
                                                        end: 325,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 326,
                                                                        end: 329,
                                                                        as_str(): "val",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 329,
                                                                    end: 330,
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
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 331,
                                                                                end: 332,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 332,
                                                                end: 333,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                    ),
                                                                    start: 334,
                                                                    end: 341,
                                                                    as_str(): "default",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 343,
                                                                            end: 344,
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 325,
                                            end: 345,
                                            as_str(): "(val: T, default: T)",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: Some(
                                        WhereClause {
                                            where_token: WhereToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 346,
                                                    end: 351,
                                                    as_str(): "where",
                                                },
                                            },
                                            bounds: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    WhereBound {
                                                        ty_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 356,
                                                                end: 357,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 357,
                                                                end: 358,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        bounds: Traits {
                                                            prefix: PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 359,
                                                                            end: 361,
                                                                            as_str(): "Eq",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                            suffixes: [],
                                                        },
                                                    },
                                                ),
                                            },
                                        },
                                    ),
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
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 368,
                                                                        end: 374,
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
                                                                    lhs: MethodCall {
                                                                        target: FuncApp {
                                                                            func: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 375,
                                                                                                end: 382,
                                                                                                as_str(): "Result2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 382,
                                                                                                    end: 384,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 384,
                                                                                                        end: 386,
                                                                                                        as_str(): "Ok",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: Some(
                                                                                                    (
                                                                                                        DoubleColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                ),
                                                                                                                start: 386,
                                                                                                                end: 388,
                                                                                                                as_str(): "::",
                                                                                                            },
                                                                                                        },
                                                                                                        GenericArgs {
                                                                                                            parameters: AngleBrackets {
                                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 388,
                                                                                                                        end: 389,
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
                                                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 389,
                                                                                                                                                end: 390,
                                                                                                                                                as_str(): "T",
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
                                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 390,
                                                                                                                                    end: 391,
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
                                                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 392,
                                                                                                                                            end: 393,
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
                                                                                                                },
                                                                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 393,
                                                                                                                        end: 394,
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                            ),
                                                                                                            start: 395,
                                                                                                            end: 398,
                                                                                                            as_str(): "val",
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
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 399,
                                                                                    as_str(): "(val)",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 399,
                                                                                end: 400,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        path_seg: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 400,
                                                                                    end: 409,
                                                                                    as_str(): "unwrap_or",
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
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 410,
                                                                                                        end: 417,
                                                                                                        as_str(): "default",
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
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 409,
                                                                                end: 418,
                                                                                as_str(): "(default)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 419,
                                                                            end: 421,
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
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 422,
                                                                                        end: 425,
                                                                                        as_str(): "val",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 374,
                                                            end: 426,
                                                            as_str(): "(Result2::Ok::<T, T>(val).unwrap_or(default) == val)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 426,
                                                            end: 427,
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
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 432,
                                                                        end: 438,
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
                                                                    lhs: MethodCall {
                                                                        target: FuncApp {
                                                                            func: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                ),
                                                                                                start: 439,
                                                                                                end: 446,
                                                                                                as_str(): "Result2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                    ),
                                                                                                    start: 446,
                                                                                                    end: 448,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 448,
                                                                                                        end: 451,
                                                                                                        as_str(): "Err",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: Some(
                                                                                                    (
                                                                                                        DoubleColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                ),
                                                                                                                start: 451,
                                                                                                                end: 453,
                                                                                                                as_str(): "::",
                                                                                                            },
                                                                                                        },
                                                                                                        GenericArgs {
                                                                                                            parameters: AngleBrackets {
                                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 453,
                                                                                                                        end: 454,
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
                                                                                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 454,
                                                                                                                                                end: 455,
                                                                                                                                                as_str(): "T",
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
                                                                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 455,
                                                                                                                                    end: 456,
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
                                                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 457,
                                                                                                                                            end: 458,
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
                                                                                                                },
                                                                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 458,
                                                                                                                        end: 459,
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
                                                                                        Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                            ),
                                                                                                            start: 460,
                                                                                                            end: 463,
                                                                                                            as_str(): "val",
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
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 459,
                                                                                    end: 464,
                                                                                    as_str(): "(val)",
                                                                                },
                                                                            },
                                                                        },
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 464,
                                                                                end: 465,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        path_seg: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 465,
                                                                                    end: 474,
                                                                                    as_str(): "unwrap_or",
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
                                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                                        ),
                                                                                                        start: 475,
                                                                                                        end: 482,
                                                                                                        as_str(): "default",
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
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 474,
                                                                                end: 483,
                                                                                as_str(): "(default)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 486,
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
                                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                        ),
                                                                                        start: 487,
                                                                                        end: 494,
                                                                                        as_str(): "default",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 438,
                                                            end: 495,
                                                            as_str(): "(Result2::Err::<T, T>(val).unwrap_or(default) == default)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 495,
                                                            end: 496,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 362,
                                        end: 498,
                                        as_str(): "{\n    assert(Result2::Ok::<T, T>(val).unwrap_or(default) == val);\n    assert(Result2::Err::<T, T>(val).unwrap_or(default) == default);\n}",
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 500,
                                            end: 502,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 503,
                                            end: 507,
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
                                            src (ptr): 0x00007fe0f97a4fa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                            ),
                                            start: 507,
                                            end: 509,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                    ),
                                                    start: 510,
                                                    end: 512,
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
                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                ),
                                                                start: 513,
                                                                end: 517,
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
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 522,
                                                                        end: 536,
                                                                        as_str(): "test_unwrap_or",
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
                                                            value_separator_pairs: [
                                                                (
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 537,
                                                                                    end: 541,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 541,
                                                                            end: 542,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 543,
                                                                                end: 547,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 536,
                                                            end: 548,
                                                            as_str(): "(true, true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 548,
                                                            end: 549,
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
                                                                        src (ptr): 0x00007fe0f97a4fa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                        ),
                                                                        start: 552,
                                                                        end: 566,
                                                                        as_str(): "test_unwrap_or",
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
                                                            value_separator_pairs: [
                                                                (
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f97a4fa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                    ),
                                                                                    start: 567,
                                                                                    end: 571,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                            ),
                                                                            start: 571,
                                                                            end: 572,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f97a4fa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                                                ),
                                                                                start: 573,
                                                                                end: 578,
                                                                                as_str(): "false",
                                                                            },
                                                                            kind: False,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 566,
                                                            end: 579,
                                                            as_str(): "(true, false)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 579,
                                                            end: 580,
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
                                                            src (ptr): 0x00007fe0f97a4fa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                                            ),
                                                            start: 585,
                                                            end: 589,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f97a4fa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRNYFy08/generic_result_method/src/main.sw",
                                        ),
                                        start: 518,
                                        end: 591,
                                        as_str(): "{\n  test_unwrap_or(true, true);\n  test_unwrap_or(true, false);\n\n\n  true\n}",
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
